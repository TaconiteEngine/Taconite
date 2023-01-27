use crate::EventHandler;
use crate::TextureManager;
use crate::WindowConfig;
use crate::World;
use log::info;
use sdl2::{event::Event, image::InitFlag, keyboard::Keycode, pixels::Color};
use std::sync::{Arc, Mutex};

pub struct Renderer<'a> {
    event_handler: EventHandler,
    pub texture_manager: Option<TextureManager<'a>>,
}

impl<'a> Renderer<'a> {
    pub fn new(world: Arc<Mutex<World>>) -> Renderer<'a> {
        info!("Creating a renderer");

        Renderer {
            event_handler: EventHandler::new(world),
            texture_manager: None,
        }
    }

    pub fn start_window(&mut self, window_config: WindowConfig) -> Result<(), String> {
        info!("Starting a window");

        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let _image_ctx = sdl2::image::init(InitFlag::PNG);

        let window = video
            .window(window_config.name, 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = match window_config.vsync {
            true => window.into_canvas().present_vsync().build(),
            false => window.into_canvas().build(),
        }
        .map_err(|e| e.to_string())?;

        self.texture_manager = Some(TextureManager::new(canvas.texture_creator()));

        canvas.set_draw_color(Color::RGB(46, 52, 64));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.event_handler.update();
            self.event_handler.draw(&mut canvas);

            canvas.present();
        }

        Ok(())
    }
}
