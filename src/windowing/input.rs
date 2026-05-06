use std::collections::HashSet;

use winit::keyboard::KeyCode;

#[derive(Debug, Default)]
pub struct InputState {
    held: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

impl InputState {
    pub fn tick(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn press(&mut self, key: KeyCode) {
        self.held.insert(key);
        self.just_pressed.insert(key);
    }

    pub fn release(&mut self, key: KeyCode) {
        self.held.remove(&key);
        self.just_released.insert(key);
    }

    pub fn is_held(&self, key: KeyCode) -> bool {
        self.held.contains(&key)
    }

    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn is_just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }
}
