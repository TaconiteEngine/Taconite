use taconite::*;

struct PrintTransformSystem {}

impl System for PrintTransformSystem {
    fn update(
        &mut self,
        manager: &mut EntityManager,
        _accessor: &mut EntityIdAccessor,
        _input_handler: &InputHandler,
    ) -> Option<()> {
        for transform in manager.borrow_components::<Transform>()?.iter() {
            println!(
                "Position:\nx: {} y: {} z: {}\n\nRotation:\nx: {} y: {} z: {}\n",
                transform.position.x,
                transform.position.y,
                transform.position.z,
                transform.rotation.x,
                transform.rotation.y,
                transform.rotation.z
            )
        }

        Some(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    taconite::init_logging();

    let mut taconite = Taconite::default();

    let entity = taconite.create_entity();

    taconite.add_component_to_entity(entity, Transform::default());
    taconite.add_component_to_entity(entity, Shape::default());

    taconite.add_system(PrintTransformSystem {});

    taconite.start(WindowConfig {
        name: "Window ECS Example".into(),
        ..Default::default()
    });

    Ok(())
}
