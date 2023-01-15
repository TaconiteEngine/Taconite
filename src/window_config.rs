pub struct WindowConfig<'a> {
    pub name: &'a str,
    pub vsync: bool,
    pub width: i32,
    pub height: i32,
}

impl<'a> WindowConfig<'a> {
    pub fn new(name: &str, vsync: bool, width: i32, height: i32) -> WindowConfig {
        WindowConfig {
            name,
            vsync,
            width,
            height,
        }
    }
}
