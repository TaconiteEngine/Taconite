use crate::EventHandler;
use crate::TextureManager;
use crate::WindowConfig;
use crate::World;
use log::info;
// use sdl2::{image::InitFlag, pixels::Color};
use std::sync::{Arc, Mutex};

pub struct Renderer {
    pub(crate) event_handler: EventHandler,
    texture_manager: Option<TextureManager>,
}

impl Renderer {
    pub fn new(world: Arc<Mutex<World>>) -> Renderer {
        info!("Creating a renderer");

        Renderer {
            event_handler: EventHandler::new(world),
            texture_manager: None,
        }
    }

    pub fn start_window(&mut self, window_config: WindowConfig) -> Result<(), String> {
        Ok(())
    }
}
