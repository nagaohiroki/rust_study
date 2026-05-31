use std::collections::HashMap;
use winit::keyboard::KeyCode;
struct InputData {
    pressed: bool,
    trigger: bool,
    released: bool,
    pressed_time: f32,
}
pub struct InputManager {
    input_data: HashMap<KeyCode, InputData>,
}
impl InputManager {
    pub fn new() -> Self {
        Self {
            input_data: HashMap::new(),
        }
    }
    pub fn handle_event(&mut self, keycode: KeyCode, is_pressed: bool) {
        let data = self.input_data.entry(keycode).or_insert(InputData {
            pressed: false,
            trigger: false,
            released: false,
            pressed_time: 0.0,
        });
        if is_pressed {
            if !data.pressed {
                data.trigger = true;
            }
            data.pressed = true;
        } else {
            if data.pressed {
                data.released = true;
            }
            data.pressed = false;
            data.pressed_time = 0.0;
        }
    }
    pub fn update(&mut self, deltatime: f32) {
        for data in self.input_data.values_mut() {
            data.trigger = false;
            data.released = false;
            if data.pressed {
                data.pressed_time += deltatime;
            }
        }
    }
    pub fn pressed(&self, keycode: KeyCode) -> bool {
        self.input_data.get(&keycode).map_or(false, |d| d.pressed)
    }
    pub fn trigger(&self, keycode: KeyCode) -> bool {
        self.input_data.get(&keycode).map_or(false, |d| d.trigger)
    }
    pub fn released(&self, keycode: KeyCode) -> bool {
        self.input_data.get(&keycode).map_or(false, |d| d.released)
    }
    // pub fn pressed_time(&self, keycode: KeyCode) -> f32 {
    //     self.input_data
    //         .get(&keycode)
    //         .map_or(0.0, |d| d.pressed_time)
    // }
}
