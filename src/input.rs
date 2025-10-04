use crate::{ mat::*, eadk::input::* };

fn bind_keys(keyboard_state: &KeyboardState, pos_key: Key, neg_key: Key, update: &mut bool, value: &mut f32) {
    if keyboard_state.key_down(pos_key) {
        *update = true;
        *value = 1.0;
    }
    else if keyboard_state.key_down(neg_key) {
        *update = true;
        *value = -1.0;
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Rotate,
    Green,
    Translate
}

#[derive(Default)]
pub struct Updates {
    pub domain: bool,
    pub rotation: bool,
    pub scale: bool,
    pub redraw: bool,
    pub mode: bool,
    pub hud: bool,
    pub secret: bool,
    pub quit: bool
}

pub struct InputHandler {
    pub upd: Updates,
    pub keyboard_state: KeyboardState,
    pub rotation_direction: Vector3,
    pub n_change: f32,
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
                mode: true,
                hud: true,
                secret: false,
                quit: false
            },
            keyboard_state: KeyboardState::scan(),
            rotation_direction: Vector3::new(0.0, 0.0, 0.0),
            n_change: 0.0,
            scale_change: 0.0,
            mode: Mode::Rotate
        }
    }

    pub fn update(&mut self) {
        self.keyboard_state = KeyboardState::scan();
        self.rotation_direction = Vector3::new(0.0, 0.0, 0.0);
        self.n_change = 0.0;
        self.scale_change = 0.0;
        self.upd = Updates::default();

        if self.mode == Mode::Rotate {
            bind_keys(&self.keyboard_state, Key::Down, Key::Up, &mut self.upd.rotation, &mut self.rotation_direction.x);
            bind_keys(&self.keyboard_state, Key::Left, Key::Right, &mut self.upd.rotation, &mut self.rotation_direction.y);
            bind_keys(&self.keyboard_state, Key::Alpha, Key::Shift, &mut self.upd.rotation, &mut self.rotation_direction.z);

            bind_keys(&self.keyboard_state, Key::Plus, Key::Minus, &mut self.upd.scale, &mut self.scale_change);
        } else if self.mode == Mode::Translate {
            bind_keys(&self.keyboard_state, Key::Plus, Key::Minus, &mut self.upd.domain, &mut self.n_change);
            if self.keyboard_state.key_down(Key::Up) {
                self.upd.domain = true;
            }
            if self.keyboard_state.key_down(Key::Three) {
                self.upd.secret = true;
            }
        }
        
        if self.keyboard_state.key_down(Key::Seven) { 
            self.upd.mode = self.mode != Mode::Rotate;
            self.mode = Mode::Rotate; 
        } else if self.keyboard_state.key_down(Key::Eight) { 
            self.upd.mode = self.mode != Mode::Green;
            self.mode = Mode::Green;
        } else if self.keyboard_state.key_down(Key::Nine) { 
            self.upd.mode = self.mode != Mode::Translate;
            self.mode = Mode::Translate;
        }

        if self.keyboard_state.key_down(Key::Home) {
            self.upd.quit = true;
        }

        self.upd.redraw = self.upd.rotation || self.upd.domain || self.upd.scale || self.upd.secret;
        self.upd.hud = self.upd.mode || self.upd.redraw;
    }
}

