use glam::Mat4;
use wgpu::Color;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProjectionType {
    Perspective,
    Orthographic,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Layer {
    Default,
    UI,
}
pub struct Camera {
    pub projection_type: ProjectionType,
    pub culling_mask: Layer,
    pub is_clear: bool,
    pub clear_color: Color,
    fov: f32,
    near: f32,
    far: f32,
}
impl Camera {
    pub fn new() -> Self {
        Self {
            projection_type: ProjectionType::Perspective,
            culling_mask: Layer::Default,
            fov: 45.0,
            near: 0.1,
            far: 100.0,
            is_clear: true,
            clear_color: Color {
                r: 0.5,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
        }
    }
    pub fn new_ui() -> Self {
        Self {
            projection_type: ProjectionType::Orthographic,
            culling_mask: Layer::UI,
            fov: 45.0,
            near: 0.1,
            far: 100.0,
            is_clear: false,
            clear_color: Color {
                r: 0.5,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
        }
    }
    pub fn get_matrix(&self, width: u32, height: u32, view: Mat4) -> Mat4 {
        match self.projection_type {
            ProjectionType::Perspective => {
                let aspect = width as f32 / height as f32;
                let proj = Mat4::perspective_lh(self.fov, aspect, self.near, self.far);
                proj * view.inverse()
            }
            ProjectionType::Orthographic => {
                glam::Mat4::orthographic_lh(0.0, width as f32, height as f32, 0.0, 0.0, 1.0)
            }
        }
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
