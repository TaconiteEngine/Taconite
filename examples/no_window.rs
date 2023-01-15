use std::borrow::Borrow;

use taconite::*;

struct Position {
    x: f32,
    y: f32,
}

struct PrintPositionSystem {}
struct MovementPositionSystem {}

impl Component for Position {}

impl System for PrintPositionSystem {
    fn update(&mut self, manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        let positions = manager.borrow_components::<Position>().unwrap();

        for position in positions.iter() {
            println!("Position: x: {:<10} y: {})", position.x, position.y);
        }
    }
}

impl System for MovementPositionSystem {
    fn update(&mut self, manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        for position in manager
            .borrow_components_mut::<Position>()
            .unwrap()
            .iter_mut()
        {
            position.x += 0.05;
            position.y += 0.05;
        }
    }
}

fn main() {
    let mut taconite = Taconite::default();

    taconite.register_component::<Position>();

    let entity = taconite.create_entity();

    taconite.add_component_to_entity(entity, Position { x: 0.0, y: 0.0 });

    taconite.add_system(PrintPositionSystem {});
    taconite.add_system(MovementPositionSystem {});

    loop {
        taconite.update();
    }
}
