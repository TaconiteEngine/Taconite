pub struct Vector2 {
    x: f32,
    y: f32,
}

pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector2 {
    pub fn splat(val: f32) -> Self {
        Self { x: val, y: val }
    }
}

impl Vector3 {
    pub fn splat(val: f32) -> Self {
        Self {
            x: val,
            y: val,
            z: val,
        }
    }
}
