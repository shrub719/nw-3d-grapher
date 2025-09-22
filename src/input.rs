use crate::{mat::*, eadk::input::*};

#[derive(Default)]
pub struct Updates {
    pub domain: bool,
    pub rotation: bool,
    pub quit: bool,
    pub cont: bool
}

pub struct InputHandler {
    pub upd: Updates,
    pub keyboard_state: KeyboardState,
    pub rotation_direction: Vector3
}
impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            upd: Updates::default(),
            keyboard_state: KeyboardState::scan(),
            rotation_direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn update(&mut self) {
        self.keyboard_state = KeyboardState::scan();
        self.rotation_direction = Vector3::new(0.0, 0.0, 0.0);
        self.upd = Updates::default();

        if self.keyboard_state.key_down(Key::Left) {
            self.upd.rotation = true;
            self.rotation_direction.x = 1.0;
        }
        
        if self.keyboard_state.key_down(Key::Ok) {
            self.upd.cont = true;
        }
        if self.keyboard_state.key_down(Key::Home) {
            self.upd.quit = true;
        }
    }
}

