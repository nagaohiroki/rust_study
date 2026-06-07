use crate::camera::Camera;
use crate::ecs::World;
use crate::input_manager::InputManager;
use crate::mesh::Mesh;
use crate::primitive::Primitive;
use crate::time_manager::TimeManager;
use crate::transform::Transform;
use std::sync::Arc;
pub struct Scene {
    pub world: World,
    model: usize,
}
impl Scene {
    pub fn create_test(device: &wgpu::Device) -> Self {
        let mut world = World::default();
        let model = world.create_entity();
        world.transforms.set(model, Transform::new());
        let (vertices, indices) = Primitive::quad();
        let mesh = Mesh::new(&device, &vertices, &indices);
        world.meshes.set(model, Arc::new(mesh));
        let camera = world.create_entity();
        world.transforms.set(camera, Transform::new());
        world.cameras.set(camera, Camera::new());
        Self { world, model }
    }
    pub fn update(&mut self, input: &InputManager, time: &TimeManager) {
        if input.pressed(winit::keyboard::KeyCode::Enter) {
            println!("pressed enter");
        }
        if input.trigger(winit::keyboard::KeyCode::Enter) {
            println!("trigger enter");
        }
        if input.released(winit::keyboard::KeyCode::Enter) {
            println!("released enter");
        }
        if let Some(transform) = self.world.transforms.get_mut(self.model) {
            transform.rotation = glam::vec3(0.0, 0.0, time.time_since_start());
            if input.pressed(winit::keyboard::KeyCode::ArrowUp) {
                transform.position.y += 1.0 * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::ArrowDown) {
                transform.position.y -= 1.0 * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::ArrowLeft) {
                transform.position.x -= 1.0 * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::ArrowRight) {
                transform.position.x += 1.0 * time.delta_time();
            }
        }
    }
}
