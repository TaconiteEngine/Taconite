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
    fn update(
        &mut self,
        manager: &mut EntityManager,
        _accessor: &mut EntityIdAccessor,
        _input_handler: &InputHandler,
    ) -> Option<()> {
        for position in manager.borrow_components::<Position>()?.iter() {
            println!("Position: x: {:<10} y: {})", position.x, position.y);
        }

        Some(())
    }
}

impl System for MovementPositionSystem {
    fn update(
        &mut self,
        manager: &mut EntityManager,
        accessor: &mut EntityIdAccessor,
        _input_handler: &InputHandler,
    ) -> Option<()> {
        for id in accessor
            .borrow_ids_for_pair::<Velocity, Position>(manager)?
            .iter()
        {
            let (velocity, mut position) = manager
                .borrow_component_pair_mut::<Velocity, Position>(*id)
                .unwrap();

            position.x += velocity.x;
            position.y += velocity.y;
        }

        Some(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt().compact().without_time().finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let mut taconite = Taconite::default();

    taconite.register_component::<Position>();
    taconite.register_component::<Velocity>();

    let entity = taconite.create_entity();

    taconite.add_component_to_entity(entity, Position { x: 0.0, y: 0.0 });
    taconite.add_component_to_entity(entity, Velocity { x: 0.01, y: 0.01 });

    taconite.add_system(PrintPositionSystem {});
    taconite.add_system(MovementPositionSystem {});

    taconite.start(WindowConfig {
        name: "ECS Example".into(),
        ..Default::default()
    });

    Ok(())
}
