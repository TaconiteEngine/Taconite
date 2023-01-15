use log::info;

use crate::components::{Sprite, Transform};
use crate::ecs::*;
use crate::event_handler::EventHandler;
use crate::renderer::*;
use crate::texture_manager::TextureManager;
use crate::Shape;

use sdl2::{render::Canvas, video::Window};

pub struct Taconite<'a> {
    world: World,
    renderer: Option<Renderer>,
}

impl Renderer for Taconite<'_> {}

impl Default for Taconite<'_> {
    fn default() -> Self {
        let mut taconite = Self {
            world: World::default(),
            texture_manager: None,
        };

        taconite.register_component::<Transform>();
        taconite.register_component::<Sprite>();
        taconite.register_component::<Shape>();

        info!("Created a Taconite instance.");

        taconite
    }
}

impl Taconite<'_> {
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

    pub fn add_render_system<T: 'static + RenderSystem>(&mut self, system: T) -> &mut World {
        self.world.add_render_system(system);
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
        self.start_window().expect("Failed to start the window.");
    }
}
