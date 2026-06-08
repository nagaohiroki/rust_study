use crate::input_manager::InputManager;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::time_manager::TimeManager;
use std::sync::Arc;
use winit;
pub struct State {
    pub input: InputManager,
    pub time: TimeManager,
    scene: Scene,
    renderer: Renderer,
}
impl State {
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        let mut renderer = pollster::block_on(Renderer::new(window));
        let mut scene = Scene::create_test();
        renderer.setup_world(&mut scene.world);
        Self {
            input: InputManager::new(),
            time: TimeManager::new(),
            scene,
            renderer,
        }
    }
    pub fn resize(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(size);
    }
    pub fn input_event(&mut self, key_event: &winit::event::KeyEvent) {
        if let winit::keyboard::PhysicalKey::Code(keycode) = key_event.physical_key {
            let is_pressed = key_event.state == winit::event::ElementState::Pressed;
            self.input.handle_event(keycode, is_pressed);
        }
    }
    pub fn update(&mut self) {
        self.time.update();
        self.scene.update(&self.input, &self.time);
        self.input.update(self.time.delta_time());
        self.renderer.render(&self.scene.world);
    }
}
