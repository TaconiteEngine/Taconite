use ecs_rust::{
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};
use taconite::*;

struct PrintUpdateSystem {}

impl System for PrintUpdateSystem {
    fn update(&mut self, _manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        println!("Updated world, this system has been run.");
    }
}

fn main() {
    let mut taconite = Taconite::default();

    taconite.add_system(PrintUpdateSystem {});

    taconite.start();
}
