use crate::state::State;
use crate::{errors::*, WindowConfig};

use wgpu::SurfaceError;
use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use tracing::{warn, error};

#[derive(Default)]
pub struct WindowStarter();

impl WindowStarter {
    pub fn run(&mut self, window_config: WindowConfig, shaders_path: String) -> Result<(), Box<dyn std::error::Error>> {
        pollster::block_on(self.create_window(window_config, shaders_path))?;

        Ok(())
    }

    // TODO: Add a way to get window config back in.
    pub async fn create_window(&mut self, window_config: WindowConfig, shaders_path: String) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(window_config.width, window_config.height))
            .with_title(window_config.name)
            .build(&event_loop)
            .map_err(|_| WindowError::WindowFailure)?;

        let mut state = State::new(window).await?;

        state.pipeline_composer.new_pipeline(&shaders_path);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
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
                            WindowEvent::Resized(physical_size) => {
                                if state.resize(*physical_size).is_err() {
                                    error!("Failed to resize")
                                }
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &&mut so w have to dereference it twice
                                if state.resize(**new_inner_size).is_err() {
                                    error!("Failed to resize")
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                    state.update();

                    match state.render() {
                        Ok(_) => {}
                        Err(e) => {
                            // Reconfigure the surface if it's lost or outdated
                            if let Some(err) = e.downcast_ref::<SurfaceError>() {
                                if err == &wgpu::SurfaceError::Lost || err == &wgpu::SurfaceError::Outdated {
                                    if state.resize(state.size).is_err() {
                                        error!("Failed to resize window");
                                        return;
                                    }
                                } else if err == &wgpu::SurfaceError::OutOfMemory {
                                    *control_flow = ControlFlow::Exit;
                                } else if err == &wgpu::SurfaceError::Timeout {
                                    warn!("Surface timeout");
                                }
                            } else {
                                // Not a surface error, close.
                                error!(e);
                                return;
                            }
                        }
                    }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    state.window().request_redraw();
                }
                _ => {}
            }
        });
    }
}
