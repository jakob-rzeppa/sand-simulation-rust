use crate::state::State;
use crate::{HEIGHT, WIDTH};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub(crate) struct App {
    state: Option<State>,
    cursor_position: Option<(f64, f64)>,
    left_mouse_button_pressed: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: None,
            cursor_position: None,
            left_mouse_button_pressed: false,
        }
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

        self.state = match pollster::block_on(State::new(
            window,
            vec![0u8; (WIDTH * HEIGHT) as usize],
            WIDTH,
            HEIGHT,
        )) {
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
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = Some((position.x, position.y));
                // Update mouse position buffer in GPU
                state.update_mouse_position(position.x, position.y);
            },
            WindowEvent::CursorLeft { .. } => {
                self.cursor_position = None;
                // Set mouse position outside viewport to hide the circle
                state.update_mouse_position(-1.0, -1.0);
            },
            WindowEvent::MouseInput {
                state: element_state,
                button: MouseButton::Left,
                ..
            } => {
                self.left_mouse_button_pressed = element_state == ElementState::Pressed;
            }
            WindowEvent::RedrawRequested => {
                // Add sand if mouse is held down
                if self.left_mouse_button_pressed {
                    if let Some((x, y)) = self.cursor_position {
                        state.add_sand_at_cursor(x, y);
                    }
                }

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
