use crate::InputHandler;

use super::entity_manager::{EntityIdAccessor, EntityManager};
// use sdl2::{render::Canvas, video::Window};

pub trait System {
    fn update(
        &mut self,
        _manager: &mut EntityManager,
        _accessor: &mut EntityIdAccessor,
        _input_handler: &InputHandler,
    ) -> Option<()>;
}

pub trait RenderSystem {
    fn update(
        &mut self,
        _manager: &mut EntityManager,
        _accessor: &mut EntityIdAccessor,
        // canvas: &mut Canvas<Window>,
    ) -> Option<()>;
}
