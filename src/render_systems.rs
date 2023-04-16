use crate::{ecs::RenderSystem, Shape, Shapes};
use sdl2::rect::Rect;

pub struct ShapeRenderSystem {}

impl RenderSystem for ShapeRenderSystem {
    fn update(
        &mut self,
        manager: &mut crate::EntityManager,
        _accessor: &mut crate::EntityIdAccessor,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Option<()> {
        let old_draw_color = canvas.draw_color();

        for shape in manager.borrow_components::<Shape>()?.iter() {
            match shape.shape {
                Shapes::Rectangle => {
                    canvas.set_draw_color(shape.color);

                    canvas
                        .fill_rect(Rect::new(
                            shape.position.x as i32,
                            shape.position.y as i32,
                            shape.scale.x as u32,
                            shape.scale.y as u32,
                        ))
                        .unwrap();
                }
            }
        }

        canvas.set_draw_color(old_draw_color);

        Some(())
    }
}
