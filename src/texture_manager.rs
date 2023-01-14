use std::collections::HashMap;
use std::rc::Rc;

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct TextureManager<'a> {
    texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<String, Rc<Texture<'a>>>,
}
