use crate::{
    components::{Sprite, Transform},
    renderer::*,
};
use ecs_rust::{component::Component, system::System, world::World};
use sdl2::{render::Canvas, video::Window};

pub trait EventHandler {
    fn update(&mut self);
    fn draw(&self, canvas: &mut Canvas<Window>);
}

pub struct Taconite {
    world: World,
}

impl Renderer for Taconite {}

impl EventHandler for Taconite {
    fn update(&mut self) {
        self.world.update();
    }

    fn draw(&self, _canvas: &mut Canvas<Window>) {}
}

impl Default for Taconite {
    fn default() -> Self {
        Self {
            world: World::new(),
        }
    }
}

impl Taconite {
    pub fn create_entity(&mut self) -> usize {
        self.world.create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        self.world.remove_entity(entity_id);
    }

    pub fn register_component<T: 'static + Component>(&mut self) -> &mut World {
        self.world.register_component::<T>();
        &mut self.world
    }

    pub fn add_system<T: 'static + System>(&mut self, system: T) -> &mut World {
        self.world.add_system(system);
        &mut self.world
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut World {
        self.world.add_component_to_entity(entity_id, component);
        &mut self.world
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn start(&mut self) {
        self.register_component::<Transform>();
        self.register_component::<Sprite>();

        self.start_window().expect("Failed to start the window.");
    }
}
