use crate::state::State;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub(crate) struct App {
    state: Option<State>,
}

impl Default for App {
    fn default() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attributes = Window::default_attributes();

        window_attributes.title = "Sand Simulation".into();

        let window = match event_loop.create_window(window_attributes) {
            Ok(window) => Arc::new(window),
            Err(e) => {
                log::error!("Creating window failed: {}", e.to_string());
                event_loop.exit();
                return;
            }
        };

        self.state = match pollster::block_on(State::new(window)) {
            Ok(state) => Some(state),
            Err(e) => {
                log::error!("Failed to create state: {}", e);
                event_loop.exit();
                return;
            }
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => {
                log::warn!("Window event couldn't find state");
                return;
            }
        };

        // only handle event if WindowId of event is the same as the windows
        if id != state.window.id() {
            log::warn!("Window event id does not match the window id");
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                log::debug!("Window close requested");
                event_loop.exit()
            }
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }

                // queue next redraw
                state.window.request_redraw();
            }
            _ => {}
        }
    }
}
