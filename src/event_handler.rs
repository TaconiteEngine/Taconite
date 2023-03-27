// TODO: Upgrade to OpenGL

use crate::{input_handler::InputHandler, World};
use glutin::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
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

    pub(crate) fn run(self) -> bool {
        self.event_loop.unwrap().run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => {
                    info!("Closing the window.");
                    return;
                }
                _ => {
                    warn!("Unhandled event");
                }
            }
        });

        true
    }
}
