use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let _window = WindowBuilder::new()
        .with_title("Bio Rust: First Step")
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