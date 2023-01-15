use crate::World;
use sdl2::{render::Canvas, video::Window};
use std::sync::{Arc, Mutex};

pub struct EventHandler {
    world: Arc<Mutex<World>>,
}

impl EventHandler {
    fn update(&mut self) {
        self.world.lock().unwrap().update();
    }
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.world.lock().unwrap().update_render(canvas);
    }
}
