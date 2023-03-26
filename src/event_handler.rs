// TODO: Upgrade

use crate::{input_handler::InputHandler, World};
use log::info;
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
        self.world.lock().unwrap().update();
    }

    pub fn draw(&mut self /*, canvas: &mut Canvas<Window>*/) {
        self.world.lock().unwrap().update_render(/*canvas*/);
    }

    pub(crate) fn handle_events(&mut self) -> bool {
        // for event in self.event_pump.as_mut().unwrap().poll_iter() {
        //     info!("{event:?}");

        //     return matches!(
        //         event,
        //         Event::Quit { .. }
        //             | Event::KeyDown {
        //                 keycode: Some(Keycode::Escape),
        //                 ..
        //             }
        //     );
        // }

        // false
        // }

        true
    }
}
