use glam::{mat4, Mat4, Vec3, Vec4};

pub struct Camera {
    projection: Mat4,
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub up: Vec3,
}

impl Camera {
    pub fn new(aspect: f32, fov: f32, znear: f32, zfar: f32) -> Self {
        Self {
            projection: make_perspective(aspect, fov, znear, zfar),
            pos: Vec3::new(0.0, 0.0, 0.0),
            yaw: (0.0),
            pitch: (0.0),
            up: Vec3::unit_y(),
        }
    }
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/opengl-perspective-projection-matrix
// nothing incorrect about using opengl docs with vulkan
pub fn make_perspective(aspect: f32, fov: f32, near: f32, far: f32) -> Mat4 {
    let s = (fov / 2.0 * std::f32::consts::PI / 180.0).tan();
    let d = far - near;
    let x_axis = Vec4::new(1.0 / (s * aspect), 0.0, 0.0, 0.0);
    let y_axis = Vec4::new(0.0, 1.0 / s, 0.0, 0.0);
    let z_axis = Vec4::new(0.0, 0.0, far / d, -(far * near) / d);
    let w_axis = Vec4::new(0.0, 0.0, 1.0, 0.0);

    mat4(x_axis, y_axis, z_axis, w_axis)
}
