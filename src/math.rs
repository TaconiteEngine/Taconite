/// A struct that holds a:
///     * x coordinate
///     * y coordinate
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// A struct that holds a:
///     * x coordinate
///     * y coordinate
///     * z coordinate
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn splat(val: f32) -> Self {
        Self { x: val, y: val }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn splat(val: f32) -> Self {
        Self {
            x: val,
            y: val,
            z: val,
        }
    }
}
