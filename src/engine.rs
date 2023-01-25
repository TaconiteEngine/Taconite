use log::{error, info};

use std::sync::{Arc, Mutex};

use crate::components::{Sprite, Transform};
use crate::renderer::*;
use crate::Shape;
use crate::{ecs::*, WindowConfig};

pub struct Taconite<'a> {
    world: Arc<Mutex<World>>,
    renderer: Option<Renderer<'a>>,
}

impl Default for Taconite<'_> {
    fn default() -> Self {
        let mut taconite = Self {
            world: Arc::new(Mutex::new(World::default())),
            renderer: None,
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
        self.world.lock().unwrap().create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        Mutex::lock(&self.world).unwrap().remove_entity(entity_id);
    }

    pub fn register_component<T: 'static + Component>(&mut self) {
        self.world.lock().unwrap().register_component::<T>();
    }

    pub fn add_system<T: 'static + System>(&mut self, system: T) {
        self.world.lock().unwrap().add_system(system);
    }

    pub fn add_render_system<T: 'static + RenderSystem>(&mut self, system: T) {
        self.world.lock().unwrap().add_render_system(system);
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) {
        self.world
            .lock()
            .unwrap()
            .add_component_to_entity(entity_id, component);
    }

    pub fn update(&mut self) {
        self.world.lock().unwrap().update();
    }

    pub fn start(&mut self, window_config: WindowConfig) {
        self.renderer = Some(Renderer::new(self.world.clone()));

        match self.begin(window_config) {
            Ok(v) => v,
            Err(e) => error!("Error starting window: {e}"),
        }
    }

    fn begin(&mut self, window_config: WindowConfig) -> Result<(), String> {
        self.renderer.as_mut().unwrap().start_window(window_config)
    }
}
