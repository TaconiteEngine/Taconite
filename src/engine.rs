use std::sync::{Arc, Mutex};
use tracing::{error, info};

use crate::input_handler::Key;
use crate::window_starter::*;
use crate::{ecs::*, WindowConfig};
use crate::{Shape, Sprite, Transform};

/// The main struct that holds the engine.
pub struct Taconite {
    world: Arc<Mutex<World>>,
    window_starter: WindowStarter,
}

impl Default for Taconite {
    fn default() -> Self {
        let mut taconite = Self {
            world: Arc::new(Mutex::new(World::default())),
            window_starter: WindowStarter::default(),
        };

        taconite.register_component::<Transform>();
        taconite.register_component::<Sprite>();
        taconite.register_component::<Shape>();

        info!("Created a Taconite instance.");

        taconite
    }
}

impl Taconite {
    /// Create a new entity.
    /// This will create a new entity with the ECS system and return it's ID (a usize).
    pub fn create_entity(&mut self) -> usize {
        Mutex::lock(&self.world).unwrap().create_entity()
    }

    /// Remove an entity.
    /// This takes in the ID of the entity (a usize)
    pub fn remove_entity(&mut self, entity_id: usize) {
        Mutex::lock(&self.world).unwrap().remove_entity(entity_id);
    }

    /// Register a new component.
    /// This takes in a struct with `Component` implemeneted.
    pub fn register_component<T: 'static + Component>(&mut self) {
        Mutex::lock(&self.world).unwrap().register_component::<T>();
    }

    /// Add a system to run every frame.
    /// This takes in a sruct with `System` implemented.
    pub fn add_system<T: 'static + System>(&mut self, system: T) {
        Mutex::lock(&self.world).unwrap().add_system(system);
    }

    /// Add a render system to run every frame (after normal systems are updated).
    /// This takes in a struct with `RenderSystem` implemented.
    pub fn add_render_system<T: 'static + RenderSystem>(&mut self, system: T) {
        Mutex::lock(&self.world).unwrap().add_render_system(system);
    }

    /// Add a component to an entity.
    /// This takes in an ID (a usize) and a struct that implements `Component`.
    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) {
        Mutex::lock(&self.world)
            .unwrap()
            .add_component_to_entity(entity_id, component);
    }

    pub fn is_key_down(&self, key_variant: Key) -> bool {
        todo!("Check if key is down");
    }

    /// Start the window and begin rendering and updating.
    /// This takes in a `WindowConfig` and opens the window.
    pub fn start(&mut self, window_config: WindowConfig) {
        // self.renderer = Some(Renderer::new(self.world.clone()));

        match self.begin(window_config) {
            Ok(v) => v,
            Err(e) => error!("Error starting window: {e}"),
        }
    }

    fn begin(&mut self, window_config: WindowConfig) -> Result<(), String> {
        todo!("Start the window.");
        // self.renderer.as_mut().unwrap().start_window(window_config)
    }
}
