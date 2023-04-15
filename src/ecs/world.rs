use sdl2::{render::Canvas, video::Window};

use crate::InputHandler;

use super::{
    component::Component,
    entity_manager::{EntityIdAccessor, EntityManager},
    system::{RenderSystem, System},
};

#[derive(Default)]
pub struct World {
    entity_manager: EntityManager,
    entity_id_accessor: EntityIdAccessor,
    systems: Vec<Box<dyn System>>,
    render_systems: Vec<Box<dyn RenderSystem>>,
}

impl World {
    pub fn create_entity(&mut self) -> usize {
        self.entity_manager.create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        self.entity_manager.remove_entity(entity_id);
    }

    pub fn register_component<T: 'static + Component>(&mut self) -> &mut Self {
        self.entity_manager.register::<T>();
        self
    }

    pub fn add_system<T: 'static + System>(&mut self, system: T) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn add_render_system<T: 'static + RenderSystem>(&mut self, system: T) -> &mut Self {
        self.render_systems.push(Box::new(system));
        self
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut Self {
        self.entity_manager
            .add_component_to_entity(entity_id, component);
        self
    }

    pub fn update(&mut self, input_handler: &InputHandler) {
        for system in self.systems.iter_mut() {
            system.update(
                &mut self.entity_manager,
                &mut self.entity_id_accessor,
                input_handler,
            );
            self.entity_manager.increment_frame();
        }
    }

    pub fn update_render(&mut self, canvas: &mut Canvas<Window>) {
        for render_system in self.render_systems.iter_mut() {
            render_system.update(
                &mut self.entity_manager,
                &mut self.entity_id_accessor,
                canvas,
            );

            self.entity_manager.increment_frame();
        }
    }
}
