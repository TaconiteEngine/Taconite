/// The WindowConfig struct holds the information for how the window should be created.
/// This is used later so that it is able to correctly inform the renderer on how to create the
/// window when needed
pub struct WindowConfig {
    pub name: String,
    pub fullscreen: bool,
    pub vsync: bool,
    pub width: i32,
    pub height: i32,
    pub auto_clear: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            name: String::from("Taconite window"),
            fullscreen: false,
            vsync: true,
            width: 640,
            height: 480,
            auto_clear: true,
        }
    }
}
