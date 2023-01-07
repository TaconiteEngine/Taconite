use std::collections::HashMap;

use sdl2::{
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct TextureManager<'a> {
    pub texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<String, Texture<'a>>,
}
