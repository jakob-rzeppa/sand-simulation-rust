use crate::app::App;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod state;

fn main() {
    // When wgpu hits any error, it panics with a generic message,
    // while logging the real error via the log crate.
    // This means if you don't include env_logger::init(), wgpu will fail silently.
    env_logger::init();

    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(e) => {
            log::error!("Creating EventLoop failed: {}", e);
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
            log::error!("Running app failed {}", e.to_string());
        }
    }
}
