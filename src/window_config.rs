use crate::Vector2;

pub struct WindowConfig {
    pub name: String,
    pub dimensions: Vector2,
    pub vsync: bool,
}

impl WindowConfig {
    pub fn new(name: String, vsync: bool, dimensions: Vector2) -> WindowConfig {
        WindowConfig {
            name,
            vsync,
            dimensions,
        }
    }
}
