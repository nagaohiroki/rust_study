use crate::camera::Camera;
use crate::ecs::World;
use crate::input_manager::InputManager;
use crate::mesh::Mesh;
use crate::primitive::Primitive;
use crate::time_manager::TimeManager;
use crate::transform::Transform;
use glam::Vec3;
use std::sync::Arc;
pub struct Scene {
    pub world: World,
    model: usize,
}
impl Scene {
    pub fn create_test(device: &wgpu::Device) -> Self {
        let mut world = World::default();
        let (vertices, indices) = Primitive::quad();
        let mesh = Arc::new(Mesh::new(&device, &vertices, &indices));
        let model = world.create_entity();
        world.transforms.set(model, Transform::new());
        world.meshes.set(model, mesh.clone());

        let model1 = world.create_entity();
        let transform = Transform {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: glam::vec3(0.1, 0.1, 0.1),
        };
        world.transforms.set(model1, transform);
        world.meshes.set(model1, mesh.clone());

        let camera = world.create_entity();
        world.transforms.set(camera, Transform::new());
        world.cameras.set(camera, Camera::new());
        Self { world, model }
    }
    pub fn update(&mut self, input: &InputManager, time: &TimeManager) {
        if let Some(transform) = self.world.transforms.get_mut(self.model) {
            transform.rotation = glam::vec3(0.0, 0.0, time.time_since_start());
            if input.pressed(winit::keyboard::KeyCode::KeyW) {
                transform.position.y += 1.0 * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::KeyS) {
                transform.position.y -= 1.0 * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::KeyA) {
                transform.position.x -= 1.0 * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::KeyD) {
                transform.position.x += 1.0 * time.delta_time();
            }
        }
    }
}
