use taconite::*;

struct RenderSys {}

impl RenderSystem for RenderSys {
    fn update(
        &mut self,
        manager: &mut EntityManager,
        accessor: &mut EntityIdAccessor,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        println!("Render");
    }
}

fn main() {
    let mut taconite = Taconite::default();

    taconite.add_render_system(RenderSys {});

    taconite.start();
}
