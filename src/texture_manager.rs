use std::collections::HashMap;
use std::rc::Rc;

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

/// A struct to enhouse textures.
/// This struct can load and keep textures with caching.
pub struct TextureManager<'a> {
    texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<String, Rc<Texture<'a>>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> TextureManager<'a> {
        TextureManager {
            texture_creator,
            cache: HashMap::new(),
        }
    }

    pub fn load(&'a mut self, path: &str) -> Result<Rc<Texture>, String> {
        self.cache.get(path).cloned().map_or_else(
            || {
                let resource = Rc::new(self.texture_creator.load_texture(path)?);
                self.cache.insert(path.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}
