#[derive(Clone, Debug, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
    pub _pad: f32,
}

#[derive(Clone, Debug, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub _pad: f32,
}

#[derive(Clone, Debug, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub _pad: f32,
}