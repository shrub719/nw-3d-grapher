use crate::{ mat::*, eadk::input::* };

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Rotate,
    Translate
}

#[derive(Default)]
pub struct Updates {
    pub domain: bool,
    pub rotation: bool,
    pub scale: bool,
    pub redraw: bool,
    pub quit: bool
}

pub struct InputHandler {
    pub upd: Updates,
    pub keyboard_state: KeyboardState,
    pub rotation_direction: Vector3,
    pub n_change: isize,
    pub scale_change: f32,
    pub mode: Mode
}
impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            upd: Updates {
                domain: true,
                rotation: true,
                scale: true,
                redraw: true,
                quit: false
            },
            keyboard_state: KeyboardState::scan(),
            rotation_direction: Vector3::new(0.0, 0.0, 0.0),
            n_change: 0,
            scale_change: 0.0,
            mode: Mode::Rotate
        }
    }

    pub fn update(&mut self) {
        self.keyboard_state = KeyboardState::scan();
        self.rotation_direction = Vector3::new(0.0, 0.0, 0.0);
        self.upd = Updates::default();

        if self.mode == Mode::Rotate {
            if self.keyboard_state.key_down(Key::Down) {
                self.upd.rotation = true;
                self.rotation_direction.x = 1.0;
            }
            else if self.keyboard_state.key_down(Key::Up) {
                self.upd.rotation = true;
                self.rotation_direction.x = -1.0;
            }
            if self.keyboard_state.key_down(Key::Left) {
                self.upd.rotation = true;
                self.rotation_direction.y = 1.0;
            }
            else if self.keyboard_state.key_down(Key::Right) {
                self.upd.rotation = true;
                self.rotation_direction.y = -1.0;
            }
            if self.keyboard_state.key_down(Key::Alpha) {
                self.upd.rotation = true;
                self.rotation_direction.z = 1.0;
            }
            else if self.keyboard_state.key_down(Key::Shift) {
                self.upd.rotation = true;
                self.rotation_direction.z = -1.0;
            }

            if self.keyboard_state.key_down(Key::Plus) {
                self.upd.scale = true;
                self.scale_change = 1.0;
            }
            else if self.keyboard_state.key_down(Key::Minus) {
                self.upd.scale = true;
                self.scale_change = -1.0;
            }
        } else if self.mode == Mode::Translate {
            if self.keyboard_state.key_down(Key::Plus) {
                self.upd.domain = true;
                self.n_change = 1;
            }
            else if self.keyboard_state.key_down(Key::Minus) {
                self.upd.domain = true;
                self.n_change = -1;
            }
        }
        
        if self.keyboard_state.key_down(Key::One) { self.mode = Mode::Rotate; } 
        else if self.keyboard_state.key_down(Key::Two) { self.mode = Mode::Translate }

        if self.keyboard_state.key_down(Key::Home) {
            self.upd.quit = true;
        }

        self.upd.redraw = self.upd.rotation || self.upd.domain || self.upd.scale;
    }
}

