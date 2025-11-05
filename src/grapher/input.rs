use crate::{ 
    grapher::mat::*, 
    eadk::input::*,
    constants::controls::*
};

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
    View,
    Trace,
    Domain
}

#[derive(Default)]
pub struct Updates {
    pub domain: bool,
    pub rotation: bool,
    pub scale: bool,
    pub redraw: bool,
    pub mode: bool,
    pub hud: bool,
    pub help_on: bool,
    pub help_off: bool,
    pub quit: bool
}

pub struct InputHandler {
    pub upd: Updates,
    pub keyboard_state: KeyboardState,
    pub rotation_direction: Vector3,
    pub domain_trans_direction: Vector3,
    pub scale_change: f32,
    pub domain_scale_direction: Vector3,
    pub mode: Mode,
    pub help: bool,
    pub domain_cooldown: f32,
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
                quit: false
            },
            keyboard_state: KeyboardState::scan(),
            rotation_direction: Vector3::new(0.0, 0.0, 0.0),
            domain_trans_direction: Vector3::new(0.0, 0.0, 0.0),
            scale_change: 0.0,
            domain_scale_direction: Vector3::new(0.0, 0.0, 0.0),
            mode: Mode::View,
            help: false,
            domain_cooldown: 0.1,
        }
    }

    pub fn update(&mut self) {
        self.keyboard_state = KeyboardState::scan();
        self.rotation_direction = Vector3::new(0.0, 0.0, 0.0);
        self.domain_trans_direction = Vector3::new(0.0, 0.0, 0.0);
        self.scale_change = 0.0;
        self.domain_scale_direction = Vector3::new(0.0, 0.0, 0.0);
        self.upd = Updates::default();

        if self.mode == Mode::View {
            bind_keys_directional(
                &self.keyboard_state,
                D_DOWN, D_UP,
                D_LEFT, D_RIGHT,
                D_SP_1, D_SP_2,
                &mut self.upd.rotation, 
                &mut self.rotation_direction
            );

            bind_keys(&self.keyboard_state, INCREASE, DECREASE, &mut self.upd.scale, &mut self.scale_change);

            if self.keyboard_state.key_down(RESET) {
                self.upd.rotation = true;
                self.rotation_direction.x = f32::NAN;
            }
        } 

        else if self.mode == Mode::Trace {
         
        }
        
        else if self.mode == Mode::Domain {
            if self.domain_cooldown >= 0.1 {
                if self.keyboard_state.key_down(MODIFIER) {
                    bind_keys_directional(
                        &self.keyboard_state,
                        D_RIGHT, D_LEFT,
                        D_UP, D_DOWN,
                        D_SP_1, D_SP_2,
                        &mut self.upd.domain,
                        &mut self.domain_scale_direction
                    );
                } else {
                    bind_keys_directional(
                        &self.keyboard_state,
                        D_RIGHT, D_LEFT,
                        D_UP, D_DOWN,
                        D_SP_1, D_SP_2,
                        &mut self.upd.domain, 
                        &mut self.domain_trans_direction
                    );

                    let mut scale_change = 0.0;
                    bind_keys(&self.keyboard_state, DECREASE, INCREASE, &mut self.upd.domain, &mut scale_change);
                    self.domain_scale_direction = Vector3::new(scale_change, scale_change, scale_change);
                }
            }
        }
        
        if self.keyboard_state.key_down(MODE_1) || self.keyboard_state.key_down(MODE_1B) { 
            self.upd.mode = self.mode != Mode::View;
            self.mode = Mode::View; 
        } else if self.keyboard_state.key_down(MODE_2) || self.keyboard_state.key_down(MODE_2B) { 
            self.upd.mode = self.mode != Mode::Trace;
            self.mode = Mode::Trace;
        } else if self.keyboard_state.key_down(MODE_3) || self.keyboard_state.key_down(MODE_3B) { 
            self.upd.mode = self.mode != Mode::Domain;
            self.mode = Mode::Domain;
        }

        if self.keyboard_state.key_down(HELP) {
            self.upd.help_on = self.help != true || self.upd.mode;
            self.help = true;
        } else {
            self.upd.help_off = self.help != false;
            self.help = false;
        }

        if self.keyboard_state.key_down(EXIT) {
            self.upd.quit = true;
        }

        self.upd.redraw = self.upd.rotation || self.upd.domain || self.upd.scale || self.upd.help_off;
        self.upd.hud = self.upd.mode || self.upd.redraw || self.upd.help_on;
    }
}

