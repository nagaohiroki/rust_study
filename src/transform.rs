use glam::{Mat4, Vec3};
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}
impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }
    pub fn get_matrix(&self) -> Mat4 {
        let pos = Mat4::from_translation(self.position);
        let rot = Mat4::from_rotation_x(self.rotation.x)
            * Mat4::from_rotation_y(self.rotation.y)
            * Mat4::from_rotation_z(self.rotation.z);
        let scale = Mat4::from_scale(self.scale);
        pos * rot * scale
    }
}
