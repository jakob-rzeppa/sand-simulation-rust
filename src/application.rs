use thiserror::Error;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Window not initialized while in use")]
    WindowUninitialized,
}

pub(crate) struct App {
    window: Option<Window>,
}

impl Default for App {
    fn default() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match event_loop.create_window(WindowAttributes::default().with_title("Pixel Sandbox")) {
            Ok(window) => {
                self.window = Some(window);
                if let Some(win) = &self.window {
                    win.request_redraw();
                }
            }
            Err(e) => {
                eprintln!("Creating window failed: {}", e.to_string());
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        // only handle event if WindowId of event is the same as the windows
        if Some(id) != self.window.as_ref().map(|w| w.id()) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let window = match &self.window {
                    Some(win) => win,
                    None => {
                        eprintln!("{}", AppError::WindowUninitialized);
                        event_loop.exit();
                        return; // early return from window_event
                    }
                };

                // draw your pixels here
                println!("Redraw frame");

                // queue next redraw
                window.request_redraw();
            }
            _ => {}
        }
    }
}