use super::entity_manager::{EntityIdAccessor, EntityManager};
use sdl2::{render::Canvas, video::Window};

pub trait System {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor);
}

pub trait RenderSystem {
    fn update(
        &mut self,
        manager: &mut EntityManager,
        accessor: &mut EntityIdAccessor,
        canvas: &mut Canvas<Window>,
    );
}
