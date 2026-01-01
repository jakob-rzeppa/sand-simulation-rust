use crate::app::App;
use winit::event_loop::EventLoop;

mod app;
mod state;

const MS_PER_SIMULATION: u64 = 16; // Update simulation every 16ms (~60 updates/sec)
const RADIUS_ADD_PARTICLES: u32 = 15;
const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;

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

    let mut app = App::default();
    match event_loop.run_app(&mut app) {
        Ok(_) => (),
        Err(e) => {
            log::error!("Running app failed {}", e.to_string());
        }
    }
}
