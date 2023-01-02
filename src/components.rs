use crate::ecs::*;
use crate::types::*;
//use sdl2::render::Texture;

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

impl Transform {
    pub fn new(position: Vector3) -> Self {
        Self {
            position,
            rotation: Vector3::splat(0.0),
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

pub struct Sprite /*<'a>*/ {
    //texture: Texture<'a>,
}

impl Component for Transform {}
impl Component for Sprite /*<'_>*/ {}
impl Component for Shape {}
