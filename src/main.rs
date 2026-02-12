use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let _window = WindowBuilder::new()
        .with_title("Bio Rust: First Step")
        .build(&event_loop)
        .unwrap();

    println!("Running");
    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => {
                println!("Closing");
                target.exit();
            }
            _ => {},
        }
    }).unwrap();
}