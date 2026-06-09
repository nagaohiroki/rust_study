use crate::camera::{Camera, Layer};
use crate::ecs::World;
use crate::input_manager::InputManager;
use crate::primitive::PrimitiveType;
use crate::time_manager::TimeManager;
use crate::transform::Transform;
use glam::Vec3;
pub struct Scene {
    pub world: World,
    model: usize,
}
impl Scene {
    pub fn create_test() -> Self {
        let mut world = World::default();
        let model = world.create_entity();
        world.transforms.set(model, Transform::new());
        world.primitive_type.set(model, PrimitiveType::Cube);
        world.layers.set(model, Layer::Default);

        let model1 = world.create_entity();
        let transform = Transform {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: glam::vec3(100.0, 100.0, 1.0),
        };
        world.transforms.set(model1, transform);
        world.primitive_type.set(model1, PrimitiveType::Quad);
        world.layers.set(model1, Layer::UI);

        let camera_3d = world.create_entity();
        let mut trans_3d = Transform::new();
        trans_3d.position.z = -5.0;
        trans_3d.position.y = 1.0;
        world.transforms.set(camera_3d, trans_3d);
        world.cameras.set(camera_3d, Camera::new());

        let camera_ui = world.create_entity();
        let mut ui_trans = Transform::new();
        ui_trans.position = glam::vec3(50.0, 50.0, 0.0);
        ui_trans.scale = glam::vec3(100.0, 100.0, 1.0);
        world.transforms.set(camera_ui, ui_trans);
        world.cameras.set(camera_ui, Camera::new_ui());

        Self { world, model }
    }
    pub fn update(&mut self, input: &InputManager, time: &TimeManager) {
        if let Some(transform) = self.world.transforms.get_mut(self.model) {
            transform.rotation = glam::vec3(0.0, 0.0, time.time_since_start());
            let speed = 1.0;
            if input.pressed(winit::keyboard::KeyCode::KeyW) {
                transform.position.y += speed * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::KeyS) {
                transform.position.y -= speed * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::KeyA) {
                transform.position.x -= speed * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::KeyD) {
                transform.position.x += speed * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::KeyQ) {
                transform.position.z -= speed * time.delta_time();
            }
            if input.pressed(winit::keyboard::KeyCode::KeyE) {
                transform.position.z += speed * time.delta_time();
            }
        }
    }
}
