use taconite::*;

struct RenderSys {}

impl RenderSystem for RenderSys {
    fn update(
        &mut self,
        _manager: &mut EntityManager,
        _accessor: &mut EntityIdAccessor,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        let i = (canvas.draw_color().r + 1) % 255;

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

        canvas.clear();
    }
}

fn main() {
    let mut taconite = Taconite::default();

    taconite.add_render_system(RenderSys {});

    taconite.start();
}
