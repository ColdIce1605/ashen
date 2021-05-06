use glam::{Mat4, Vec3};

#[rustfmt::skip]
#[allow(dead_code)]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = glam::const_mat4!(
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.5, 1.0]
);

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        // 1.
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = glam::Mat4::perspective_rh(
            (self.fovy).to_radians(),
            self.aspect,
            self.znear,
            self.zfar,
        );

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
