/// Components for use within the engine
use crate::ecs::*;
use crate::math::*;
use sdl2::{pixels::Color, render::Texture};

/// A component that holds position and rotation data.
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
}

/// A bunch of shapes
/// TODO: Add more shapes
pub enum Shapes {
    Rectangle,
}

/// A struct that contains a shape and a scale for a shape
pub struct Shape {
    pub shape: Shapes,
    pub position: Vector2,
    pub scale: Vector2,
    pub color: Color,
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
            position: Vector2::splat(0.0),
            scale: Vector2::splat(32.0),
            color: Color::WHITE,
        }
    }
}

/// A struct that will be used for drawing sprites to the screen
/// TODO: Add more information about sprites and ability to draw them
#[allow(dead_code)]
pub struct Sprite<'a> {
    texture: Texture<'a>,
}

impl Component for Transform {}
impl Component for Sprite<'_> {}
impl Component for Shape {}
