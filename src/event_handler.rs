use crate::{
    input_handler::{InputHandler, Key},
    World,
};
// use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump};
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub struct EventHandler {
    world: Arc<Mutex<World>>,
    // pub(crate) event_pump: Option<EventPump>,
    pub(crate) input_handler: InputHandler,
}

impl EventHandler {
    pub fn new(world: Arc<Mutex<World>>) -> EventHandler {
        EventHandler {
            world,
            // event_pump,
            input_handler: InputHandler::default(),
        }
    }

    pub fn update(&mut self) {
        self.world.lock().unwrap().update(&self.input_handler);
    }

    pub fn draw(&mut self, auto_clear: bool) {
        // if auto_clear {
        //     canvas.clear();
        // }

        self.world.lock().unwrap().update_render();
    }

    // NOTE: `handle_events` moved to `State`
}
