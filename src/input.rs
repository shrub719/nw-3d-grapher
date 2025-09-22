use crate::{mat::*, eadk::input::*};

pub struct InputHandler {
    pub update_domain: bool,
    pub update_rotation: bool,
    pub quit: bool,
    pub cont: bool,
    pub keyboard_state: KeyboardState,
    pub rotation_direction: Vector3
}
impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            update_domain: true,
            update_rotation: true,
            quit: false,
            cont: false,
            keyboard_state: KeyboardState::scan(),
            rotation_direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn update(&mut self) {
        self.keyboard_state = KeyboardState::scan();
        self.rotation_direction = Vector3::new(0.0, 0.0, 0.0);
        // replace these bools with a separate structure
        self.update_domain = false;
        self.update_rotation = false;
        self.quit = false;
        self.cont = false;

        if self.keyboard_state.key_down(Key::Left) {
            self.update_rotation = true;
            self.rotation_direction.x = 1.0;
        }
        
        if self.keyboard_state.key_down(Key::Ok) {
            self.cont = true;
        }
        if self.keyboard_state.key_down(Key::Home) {
            self.quit = true;
        }
    }
}

