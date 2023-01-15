use crate::EventHandler;
use crate::World;
use sdl2::{event::Event, image::InitFlag, keyboard::Keycode, pixels::Color};
use std::sync::{Arc, Mutex};

pub struct Renderer {
    event_handler: EventHandler,
}

impl Renderer {
    pub fn new(world: Arc<Mutex<World>>) -> Renderer {
        Renderer {
            event_handler: EventHandler::new(world),
        }
    }

    pub fn start_window(&mut self) -> Result<(), String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let _image_ctx = sdl2::image::init(InitFlag::PNG);

        let window = video
            .window("Temporary window", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

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
