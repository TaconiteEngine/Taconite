use ecs_rust::world::World;

trait EventHandler {
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
