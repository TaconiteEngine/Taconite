use std::collections::HashMap;
use std::rc::Rc;

// use sdl2::{
//     image::LoadTexture,
//     render::{Texture, TextureCreator},
//     video::WindowContext,
// };

/// A struct to enhouse textures.
/// This struct can load and keep textures with caching.
pub struct TextureManager {
    // TODO: Change this to the correct data type
    cache: HashMap<String, Rc<String>>,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            cache: HashMap::new(),
        }
    }
}
