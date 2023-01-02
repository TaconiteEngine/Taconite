use log::info;
use taconite::*;

struct RenderSys {}

impl RenderSystem for RenderSys {
    fn update(
        &mut self,
        _manager: &mut EntityManager,
        _accessor: &mut EntityIdAccessor,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        let i = canvas.draw_color().r + 1;

        info!("{}", i);

        canvas.set_draw_color(Color {
            r: i,
            g: i,
            b: i,
            a: 255,
        });

        canvas.clear();
    }
}

fn main() {
    env_logger::init();

    let mut taconite = Taconite::default();

    taconite.add_render_system(RenderSys {});

    taconite.start();
}
