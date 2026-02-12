mod universe;
mod vertex;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use bio::seq_analysis::gc::gc_content;
use wgpu::*;
use wgpu::util::DeviceExt;

use crate::universe::Universe;
use crate::vertex::{Vertex, create_grid_vertices};

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

    let mut universe = Universe::new(10, 10);
    let cell_size = 0.08;
    let mut grid_data = create_grid_vertices(&universe, cell_size);

    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&grid_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
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
    let mut cursor_pos = winit::dpi::PhysicalPosition::new(0.0, 0.0);
    let mut last_update_inst = std::time::Instant::now();

    let window_ref = &*window;

    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => {
                println!("Closing");
                target.exit();
            }

            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                cursor_pos = position;
            }

            Event::WindowEvent { 
                event: WindowEvent::MouseInput { 
                    state: winit::event::ElementState::Pressed,
                    button: winit::event::MouseButton::Left,
                    ..
                }, 
                .. 
            } => {
                let size = window_ref.inner_size();
                let x = (cursor_pos.x as f32 / size.width as f32) * 2.0 - 1.0;
                let y = (cursor_pos.y as f32 / size.height as f32) * -2.0 + 1.0;

                for row in 0..universe.rows {
                    for col in 0..universe.cols {
                        let padding = 0.02;
                        let x_offset = (col as f32 * (cell_size + padding)) - 0.6;
                        let y_offset = (row as f32 * (cell_size + padding)) - 0.6;

                        if x >= x_offset && x <= x_offset + cell_size &&
                           y >= y_offset && y <= y_offset + cell_size {
                            universe.toggle(row, col);
                            grid_data = create_grid_vertices(&universe, cell_size);
                            if !grid_data.is_empty() {
                                queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&grid_data));
                            }
                        }
                    }
                }
            }

            Event::AboutToWait => {
                if last_update_inst.elapsed() >= std::time::Duration::from_millis(200) {
                    universe.tick();
                    grid_data = create_grid_vertices(&universe, cell_size);
                    queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&grid_data));
                    last_update_inst = std::time::Instant::now();
                }
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