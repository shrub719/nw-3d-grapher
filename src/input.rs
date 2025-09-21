use crate::{mat::*, eadk::input::*};

pub struct InputHandler {
    pub update_domain: bool,
    pub update_rotation: bool,
    pub keyboard_state: KeyboardState,
    pub rotation_direction: Vector3
}
impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            update_domain: true,
            update_rotation: true,
            keyboard_state: KeyboardState::scan(),
            rotation_direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn update(&mut self) {
        self.keyboard_state = KeyboardState::scan();
    }
}

