use taconite::*;

struct ComponentRenderer {}

struct MovementSystem {}

impl System for MovementSystem {
    fn update(
        &mut self,
        manager: &mut EntityManager,
        accessor: &mut EntityIdAccessor,
        input_handler: &InputHandler,
    ) -> Option<()> {
        for transform in manager.borrow_components_mut::<Transform>()?.iter_mut() {
            transform.position.x += 1.;
            transform.position.y += 1.;
        }

        Some(())
    }
}

impl RenderSystem for ComponentRenderer {
    fn update(
        &mut self,
        manager: &mut EntityManager,
        _accessor: &mut EntityIdAccessor,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Option<()> {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        canvas.set_draw_color(Color::RGB(10, 10, 10));

        for transform in manager.borrow_components::<Transform>()?.iter() {
            canvas
                .fill_rect(Rect::new(
                    (transform.position.x - 5.0) as i32,
                    (transform.position.y - 5.0) as i32,
                    10,
                    10,
                ))
                .expect("Failed to draw to the canvas.");
        }

        Some(())
    }
}

fn main() {
    pretty_env_logger::init();

    let mut taconite = Taconite::default();

    let entity = taconite.create_entity();
    taconite.add_component_to_entity(
        entity,
        Transform::new(Vector3 {
            x: 10.0,
            y: 10.0,
            z: 0.0,
        }),
    );

    taconite.add_system(MovementSystem {});
    taconite.add_render_system(ComponentRenderer {});

    taconite.start(WindowConfig {
        name: "Render Test Example",
        vsync: true,
        width: 800,
        height: 600,
    });
}
