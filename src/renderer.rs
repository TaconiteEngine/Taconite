use crate::EventHandler;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

pub trait Renderer: EventHandler {
    fn start_window(&mut self) -> Result<(), String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let window = video
            .window("Temporary window", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

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

            canvas.clear();
            canvas.present();

            self.update();
        }

        Ok(())
    }
}
