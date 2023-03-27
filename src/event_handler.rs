// TODO: Upgrade to OpenGL

use crate::renderer::Renderer;
use crate::{input_handler::InputHandler, World};
use glow::Context;
use glow::HasContext;
use glow::NativeVertexArray;
use glow::Program;
use glutin::event::WindowEvent;
use glutin::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    ContextWrapper, PossiblyCurrent,
};
use log::{info, warn};
use std::sync::{Arc, Mutex};

pub struct EventHandler {
    world: Arc<Mutex<World>>,
    pub(crate) event_loop: Option<EventLoop<()>>,
    pub(crate) input_handler: InputHandler,
}

impl EventHandler {
    pub fn new(world: Arc<Mutex<World>>, event_loop: Option<EventLoop<()>>) -> EventHandler {
        EventHandler {
            world,
            event_loop,
            input_handler: InputHandler::default(),
        }
    }

    pub fn update(&mut self) {
        self.world.lock().unwrap().update();
    }

    pub fn draw(&mut self /*, canvas: &mut Canvas<Window>*/) {
        self.world.lock().unwrap().update_render(/*canvas*/);
    }

    pub(crate) unsafe fn run(
        self,
        gl: Context,
        window: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    ) {
        self.event_loop.unwrap().run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => {
                    // Close the loop.

                    return;
                }
                Event::MainEventsCleared => {
                    // Redraw the window.

                    window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    // Draw to the screen

                    gl.clear(glow::COLOR_BUFFER_BIT);
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    window.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        // Resize the window

                        window.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => {
                        // Delete the program
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
