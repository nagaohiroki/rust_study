use glam::{Mat4, Vec3};
pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
    z_near: f32,
    z_far: f32,
}
impl Camera {
    pub fn new() -> Self {
        Self {
            position: glam::vec3(0.0, 1.5, -2.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }
    pub fn get_matrix(&self, width: u32, height: u32) -> Mat4 {
        let view = Mat4::look_at_lh(self.position, self.target, self.up);
        let aspect_ratio = width as f32 / height as f32;
        let proj = Mat4::perspective_lh(self.fov, aspect_ratio, self.z_near, self.z_far);
        proj * view
    }
}
