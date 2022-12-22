use taconite::*;

struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    x: f32,
    y: f32,
}

struct PrintPositionSystem {}
struct MovementPositionSystem {}

impl Component for Position {}
impl Component for Velocity {}

impl System for PrintPositionSystem {
    fn update(&mut self, manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        let positions = manager.borrow_components::<Position>().unwrap();

        for position in positions.iter() {
            println!("Position: x: {:<10} y: {})", position.x, position.y);
        }
    }
}

impl System for MovementPositionSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let entity_ids = accessor
            .borrow_ids_for_pair::<Velocity, Position>(manager)
            .unwrap();

        for id in entity_ids.iter() {
            let (velocity, mut position) = manager
                .borrow_component_pair_mut::<Velocity, Position>(*id)
                .unwrap();

            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}

fn main() {
    let mut taconite = Taconite::default();

    taconite
        .register_component::<Position>()
        .register_component::<Velocity>();

    let entity = taconite.create_entity();

    taconite
        .add_component_to_entity(entity, Position { x: 0.0, y: 0.0 })
        .add_component_to_entity(entity, Velocity { x: 0.01, y: 0.01 });

    taconite
        .add_system(PrintPositionSystem {})
        .add_system(MovementPositionSystem {});

    loop {
        taconite.update();
    }
}
