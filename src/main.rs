use winit::event_loop::{ControlFlow, EventLoop};
use crate::application::App;

mod application;

fn main() {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(e) => {
            eprintln!("Creating EventLoop failed: {}", e);
            return;
        }
    };

    // ControlFlow::Poll continuously runs the event loop,
    // even if the OS hasn't dispatched any events.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    match event_loop.run_app(&mut app) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Running app failed {}", e.to_string());
        }
    }
}
