use crate::WindowConfig;

use thiserror::*;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

#[derive(Default)]
pub struct WindowStarter();

// TODO: Remove allowing dead code
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum WindowError {
    #[error("There was no GPU detected.")]
    NoGPU,
    #[error("Creating a new window with Winit failed.")]
    WindowFailure,
    #[error("Creating a new wgpu surface failed.")]
    SurfaceFailure,
}

impl WindowStarter {
    pub fn create_window(&mut self, window_config: WindowConfig) -> Result<(), WindowError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .map_err(|_| WindowError::WindowFailure)?;

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        });
    }
}
