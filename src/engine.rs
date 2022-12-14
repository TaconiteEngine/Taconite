use ecs_rust::{component::Component, system::System, world::World};

pub trait EventHandler {
    fn update(&mut self);
    fn draw(&self);
}

pub struct Taconite {
    world: World,
}

impl EventHandler for Taconite {
    fn update(&mut self) {
        self.world.update();
    }

    fn draw(&self) {
        println!("Draw");
    }
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
}
