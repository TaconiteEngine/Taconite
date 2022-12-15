use crate::types::*;
use ecs_rust::component::Component;
use sdl2::render::Texture;

pub struct Transform {
    position: Vector3,
    rotation: Vector3,
}

pub struct Sprite<'a> {
    texture: Texture<'a>,
}

impl Component for Transform {}
impl Component for Sprite<'_> {}
