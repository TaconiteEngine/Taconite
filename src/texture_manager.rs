use std::collections::HashMap;
use std::rc::Rc;

use wgpu::Texture;

// use sdl2::{
//     image::LoadTexture,
//     render::{Texture, TextureCreator},
//     video::WindowContext,
// };

/// A struct to enhouse textures.
/// This struct can load and keep textures with caching.
#[derive(Default)]
pub struct TextureManager {
    // texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<String, Rc<Texture>>,
}

impl TextureManager {
    pub fn load(&mut self, path: &str) -> Result<Rc<Texture>, String> {
        self.cache.get(path).cloned().map_or_else(
            || {
                // let resource = Rc::new(self.texture_creator.load_texture(path)?);
                // self.cache.insert(path.into(), resource.clone());
                // Ok(resource)

                todo!("Implement texture manager?");
            },
            Ok,
        )
    }
}
