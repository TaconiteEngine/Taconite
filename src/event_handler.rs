use crate::World;
use log::info;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump};
use std::sync::{Arc, Mutex};

pub struct EventHandler {
    world: Arc<Mutex<World>>,
    pub(crate) event_pump: Option<EventPump>,
}

impl EventHandler {
    pub fn new(world: Arc<Mutex<World>>, event_pump: Option<EventPump>) -> EventHandler {
        EventHandler { world, event_pump }
    }

    pub fn update(&mut self) {
        self.world.lock().unwrap().update();
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.world.lock().unwrap().update_render(canvas);
    }

    pub(crate) fn handle_events(&mut self) -> bool {
        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            return match event {
                Event::Quit { .. } => true,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => true,
                _ => false,
            };
        }

        false
    }
}
