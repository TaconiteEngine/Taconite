use crate::types::*;
use ecs_rust::component::Component;
use sdl2::render::Texture;

pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
}

pub enum Shapes {
    Rectangle,
}

pub struct Shape {
    pub shape: Shapes,
    pub scale: Vector2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            shape: Shapes::Rectangle,
            scale: Vector2::new(128., 128.),
        }
    }
}

pub struct Sprite<'a> {
    texture: Texture<'a>,
}

impl Component for Transform {}
impl Component for Sprite<'_> {}
impl Component for Shape {}
