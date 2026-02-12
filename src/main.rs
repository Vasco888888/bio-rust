use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use bio::seq_analysis::gc::gc_content;

use wgpu::*;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                }
            ]
        }
    }
}

fn create_grid_vertices(dna: &[u8], rows: u32, cols: u32, cell_size: f32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let padding = 0.02;

    for (i, &base) in dna.iter().enumerate() {
        if i >= (rows * cols) as usize { break; }

        let row = i as u32 / cols;
        let col = i as u32 % cols;

        let x_offset = (col as f32 * (cell_size + padding)) - 0.6;
        let y_offset = (row as f32 * (cell_size + padding)) - 0.6;

        let color = match base {
            b'A' => [1.0, 0.0, 0.0], 
            b'C' => [0.0, 1.0, 0.0], 
            b'G' => [0.0, 0.0, 1.0], 
            b'T' => [1.0, 1.0, 0.0], 
            _ => [0.5, 0.5, 0.5],    
        };

        vertices.extend_from_slice(&[
            Vertex { position: [x_offset, y_offset + cell_size], color },
            Vertex { position: [x_offset, y_offset], color },
            Vertex { position: [x_offset + cell_size, y_offset], color },

            Vertex { position: [x_offset, y_offset + cell_size], color },
            Vertex { position: [x_offset + cell_size, y_offset], color },
            Vertex { position: [x_offset + cell_size, y_offset + cell_size], color },
        ]);
    }
    vertices
}

fn main() {
    let dna = b"GATCCAGATCGATCCGATCGATC";
    let gc = gc_content(dna);
    println!("--- Bio Analysis ---");
    println!("Sequence: {}", std::str::from_utf8(dna).unwrap());
    println!("GC-Content: {:.2}%", gc * 100.0);
    println!("--------------------");

    let event_loop = EventLoop::new().unwrap();

    let instance = Instance::default();

    let window = Box::leak(Box::new(
        WindowBuilder::new()
            .with_title("Bio Rust")
            .build(&event_loop)
            .unwrap()
    ));

    let surface = instance.create_surface(&*window).unwrap();

    let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    })).expect("Failed to request adapter");

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor{
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::default(),
            memory_hints: Default::default(),
        },
        None,
    )).expect("Failed to request device");

    let size = window.inner_size();
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats[0];
    let config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    let grid_data = create_grid_vertices(dna, 5, 5, 0.15);

    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&grid_data),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"), 
            buffers: &[Vertex::desc()],  
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, 
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    println!("Running");

    let mut color_toggle = false;

    let window_ref = &*window;

    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => {
                println!("Closing");
                target.exit();
            }

            Event::AboutToWait => {
                window_ref.request_redraw();
            }

            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let output = surface.get_current_texture().unwrap();
                let view = output.texture.create_view(&TextureViewDescriptor::default());

                let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                {
                    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(Color { 
                                    r: if color_toggle { 0.15 } else { 0.05 }, 
                                    g: 0.05, 
                                    b: if !color_toggle { 0.15 } else { 0.05 }, 
                                    a: 1.0 
                                }),
                                store: StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    });

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.draw(0..grid_data.len() as u32, 0..1);
                }

                queue.submit(std::iter::once(encoder.finish()));
                output.present();
            }

            Event::WindowEvent { 
                event: WindowEvent::KeyboardInput { 
                    event: input, 
                    .. 
                }, 
                .. 
            } => {
                if input.state == winit::event::ElementState::Pressed {
                    color_toggle = !color_toggle;

                    if color_toggle {
                        println!("Background: Dim Red");
                    } else {
                        println!("Background: Dim Blue");
                    }
                }
            }
            _ => {},
        }
    }).unwrap();
}