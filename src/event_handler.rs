use crate::World;
use log::info;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};
use std::sync::{Arc, Mutex};

pub struct EventHandler {
    world: Arc<Mutex<World>>,
}

impl EventHandler {
    pub fn new(world: Arc<Mutex<World>>) -> EventHandler {
        EventHandler { world }
    }

    pub fn update(&mut self) {
        self.world.lock().unwrap().update();
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.world.lock().unwrap().update_render(canvas);
    }

    pub(crate) fn event(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. } => true,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => true,
            _ => false,
        }
    }
}
