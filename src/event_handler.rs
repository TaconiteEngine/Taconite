use crate::{
    input_handler::{InputHandler, Key},
    World,
};
use log::info;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump};
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub struct EventHandler {
    world: Arc<Mutex<World>>,
    pub(crate) event_pump: Option<EventPump>,
    pub(crate) input_handler: InputHandler,
}

impl EventHandler {
    pub fn new(world: Arc<Mutex<World>>, event_pump: Option<EventPump>) -> EventHandler {
        EventHandler {
            world,
            event_pump,
            input_handler: InputHandler::default(),
        }
    }

    pub fn update(&mut self) {
        self.world.lock().unwrap().update(&self.input_handler);
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.world.lock().unwrap().update_render(canvas);
    }

    // TODO: Use SDL keycodes.
    pub(crate) fn handle_events(&mut self) -> bool {
        let mut window_should_close = false;

        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            info!("{event:?}");

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => window_should_close = true,

                // W down.
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    println!("W down");
                    self.input_handler.add_key(Key::W)
                }

                // A down.
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.input_handler.add_key(Key::A);
                    println!("A down");
                }

                // S down.
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    self.input_handler.add_key(Key::S);
                    println!("S down");
                }

                // D down.
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.input_handler.add_key(Key::D);
                    println!("D down");
                }

                // W up.
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    self.input_handler.remove_key(Key::W);
                    println!("W up");
                }

                // A up.
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.input_handler.remove_key(Key::A);
                    println!("A up");
                }

                // S up.
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    self.input_handler.remove_key(Key::S);
                    println!("S up");
                }

                // D up.
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.input_handler.remove_key(Key::D);
                    println!("D up");
                }

                _ => {}
            };
        }

        window_should_close
    }
}
