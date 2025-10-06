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

fn bind_keys_directional(
    keyboard_state: &KeyboardState, 
    x_pos_key: Key, x_neg_key: Key, 
    y_pos_key: Key, y_neg_key: Key, 
    z_pos_key: Key, z_neg_key: Key, 
    update: &mut bool, 
    vector: &mut Vector3
) {
    bind_keys(keyboard_state, x_pos_key, x_neg_key, update, &mut vector.x);
    bind_keys(keyboard_state, y_pos_key, y_neg_key, update, &mut vector.y);
    bind_keys(keyboard_state, z_pos_key, z_neg_key, update, &mut vector.z);
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Rotate,
    Green,
    Translate
}

#[derive(Default)]
pub struct Updates {
    // TODO: use mod? or embedded struct
    pub domain: bool,
    pub rotation: bool,
    pub scale: bool, // TODO: scale -> zoom
    pub redraw: bool,
    pub mode: bool,
    pub hud: bool,
    pub help_on: bool,
    pub help_off: bool,
    pub load_obj: bool,
    pub quit: bool
}

pub struct InputHandler {
    pub upd: Updates,
    pub keyboard_state: KeyboardState,
    pub rotation_direction: Vector3,
    pub domain_direction: Vector3,
    pub scale_change: f32,
    pub domain_scale_change: f32,
    pub mode: Mode,
    pub help: bool,
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
                help_on: false,
                help_off: false,
                load_obj: false,
                quit: false
            },
            keyboard_state: KeyboardState::scan(),
            rotation_direction: Vector3::new(0.0, 0.0, 0.0),
            domain_direction: Vector3::new(0.0, 0.0, 0.0),
            scale_change: 0.0,
            domain_scale_change: 0.0,
            mode: Mode::Rotate,
            help: false,
        }
    }

    pub fn update(&mut self) {
        self.keyboard_state = KeyboardState::scan();
        self.rotation_direction = Vector3::new(0.0, 0.0, 0.0);
        self.domain_direction = Vector3::new(0.0, 0.0, 0.0);
        self.scale_change = 0.0;
        self.domain_scale_change = 0.0;
        self.upd = Updates::default();

        if self.mode == Mode::Rotate {
            bind_keys_directional(
                &self.keyboard_state,
                Key::Down, Key::Up,
                Key::Left, Key::Right,
                Key::Alpha, Key::Shift,
                &mut self.upd.rotation, 
                &mut self.rotation_direction
            );

            bind_keys(&self.keyboard_state, Key::Plus, Key::Minus, &mut self.upd.scale, &mut self.scale_change);

            if self.keyboard_state.key_down(Key::Backspace) {
                self.upd.rotation = true;
                self.rotation_direction.x = f32::NAN;
            }
        } 
        
        else if self.mode == Mode::Translate {
            bind_keys_directional(
                &self.keyboard_state,
                Key::Left, Key::Right,
                Key::Up, Key::Down,
                Key::Alpha, Key::Shift,
                &mut self.upd.domain, 
                &mut self.domain_direction
            );

            bind_keys(&self.keyboard_state, Key::Plus, Key::Minus, &mut self.upd.domain, &mut self.domain_scale_change);

            if self.keyboard_state.key_down(Key::Backspace) {
                self.upd.domain = true;
            }
            if self.keyboard_state.key_down(Key::Power) {
                self.upd.load_obj = true;
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

        if self.keyboard_state.key_down(Key::Toolbox) {
            self.upd.help_on = self.help != true || self.upd.mode;
            self.help = true;
        } else {
            self.upd.help_off = self.help != false;
            self.help = false;
        }

        if self.keyboard_state.key_down(Key::Home) {
            self.upd.quit = true;
        }

        self.upd.redraw = self.upd.rotation || self.upd.domain || self.upd.scale || self.upd.load_obj || self.upd.help_off;
        self.upd.hud = self.upd.mode || self.upd.redraw || self.upd.help_on;
    }
}

