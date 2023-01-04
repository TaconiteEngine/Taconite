use std::collections::HashMap;

use sdl2::{
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct TextureManager<'a> {
    texture_creator: &'a Option<TextureCreator<WindowContext>>,
    textures: HashMap<String, Texture<'a>>,
}

impl Default for TextureManager<'_> {
    fn default() -> Self {
        Self {
            texture_creator: &None,
            textures: HashMap::default(),
        }
    }
}
