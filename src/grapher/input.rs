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

#[derive(Default)]
pub struct Updates {
    // TODO: use mod? or embedded struct
    pub rotation: bool,
    pub scale: bool,
    pub redraw: bool,
    pub quit: bool
}

pub struct InputHandler {
    pub upd: Updates,
    pub keyboard_state: KeyboardState,
    pub rotation_direction: Vector3,
    pub scale_change: f32,
    pub shading: bool,
}
impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            upd: Updates {
                rotation: true,
                scale: true,
                redraw: true,
                quit: false
            },
            keyboard_state: KeyboardState::scan(),
            rotation_direction: Vector3::new(0.0, 0.0, 0.0),
            scale_change: 0.0,
            shading: true,
        }
    }

    pub fn update(&mut self) {
        let prev_switch = self.keyboard_state.key_down(SWITCH);
        self.keyboard_state = KeyboardState::scan();
        self.rotation_direction = Vector3::new(0.0, 0.0, 0.0);
        self.scale_change = 0.0;
        self.upd = Updates::default();

        bind_keys_directional(
            &self.keyboard_state,
            D_DOWN, D_UP,
            D_LEFT, D_RIGHT,
            D_SP_1, D_SP_2,
            &mut self.upd.rotation, 
            &mut self.rotation_direction
        );

        bind_keys(&self.keyboard_state, INCREASE, DECREASE, &mut self.upd.scale, &mut self.scale_change);

        if self.keyboard_state.key_down(SWITCH) && !prev_switch {
            self.shading = !self.shading;
            self.upd.rotation = true;
        }

        if self.keyboard_state.key_down(EXIT) {
            self.upd.quit = true;
        }

        self.upd.redraw = self.upd.rotation || self.upd.scale;
    }
}

