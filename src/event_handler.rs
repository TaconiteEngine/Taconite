use sdl2::{render::Canvas, video::Window};

pub trait EventHandler {
    fn update(&mut self);
    fn draw(&mut self, canvas: &mut Canvas<Window>);
}
