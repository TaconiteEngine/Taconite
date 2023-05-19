use crate::ecs::RenderSystem;
// use sdl2::rect::Rect;

pub struct ShapeRenderSystem {}

impl RenderSystem for ShapeRenderSystem {
    fn update(
        &mut self,
        manager: &mut crate::EntityManager,
        _accessor: &mut crate::EntityIdAccessor,
    ) -> Option<()> {
        todo!("Setup the Shape Render System");
    }
}
