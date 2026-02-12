use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use wgpu::*;

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

const VERTICES: &[Vertex] = &[
    // Triangle 1
    Vertex { position: [-0.5,  0.5], color: [1.0, 0.0, 0.0] }, 
    Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0] },
    
    // Triangle 2
    Vertex { position: [-0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5], color: [1.0, 1.0, 0.0] },
];

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let _window = WindowBuilder::new()
        .with_title("Bio Rust")
        .build(&event_loop)
        .unwrap();

    println!("Running");

    let mut color_toggle = false;

    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => {
                println!("Closing");
                target.exit();
            }

            Event::WindowEvent { event: WindowEvent::KeyboardInput { .. }, .. } => {
                color_toggle = !color_toggle;

                if color_toggle {
                    println!("The grid is now RED");
                } else {
                    println!("The grid is now BLUE");
                }
            }
            _ => {},
        }
    }).unwrap();
}