pub mod settings {
    pub const ROTATION_SPEED: f32 = 1.5;
    pub const SCALE_SPEED: f32 = 0.5;
}

pub mod palette {
    use crate::eadk::Color;

    pub const ORANGE: Color = Color::from_rgb(255, 183, 52);
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);
}

pub mod graphics {
    pub const SCREEN_WIDTH: u16 = 320;
    pub const SCREEN_HEIGHT: u16 = 240;

    pub const HUD_HEIGHT: u16 = 0;

    pub const MARGIN_TOP: u16 = 18;
    pub const MARGIN_BOTTOM: u16 = HUD_HEIGHT;

    // 2 is possible with margins and very few triangles... i kinda wanna try stretch it further
    pub const FB_TILE: u16 = 3;

    pub const FB_WIDTH: u16 = SCREEN_WIDTH / FB_TILE;
    pub const FB_HEIGHT: u16 = (SCREEN_HEIGHT - (MARGIN_TOP + MARGIN_BOTTOM)) / FB_TILE;
    pub const FB_WIDTH_SIZE: usize = FB_WIDTH as usize;
    pub const FB_HEIGHT_SIZE: usize = FB_HEIGHT as usize;

    use crate::eadk::Color;
    use super::palette::*;
    pub const BG: Color = WHITE;
}

pub mod limits {
    pub const MAX_TRIS: usize = 2000;
    // pub const MAX_LINES: usize = 20;  // TODO: add lines
}

pub mod controls {
    use crate::eadk::input::Key;

    pub const D_DOWN: Key =     Key::Down;
    pub const D_UP: Key =       Key::Up;
    pub const D_LEFT: Key =     Key::Left;
    pub const D_RIGHT: Key =    Key::Right;
    pub const D_SP_1: Key =     Key::Alpha;
    pub const D_SP_2: Key =     Key::Shift;

    pub const INCREASE: Key =   Key::Plus;
    pub const DECREASE: Key =   Key::Minus;
    pub const MODIFIER: Key =   Key::Multiplication;

    pub const CONFIRM: Key =    Key::OK;
    pub const BACK: Key =       Key::Back;

    pub const HELP: Key =       Key::Toolbox;
    pub const RESET: Key =      Key::Backspace;

    pub const MODE_1: Key =     Key::Seven;
    pub const MODE_1B: Key =    Key::One;
    pub const MODE_2: Key =     Key::Eight;
    pub const MODE_2B: Key =    Key::Two;
    pub const MODE_3: Key =     Key::Nine;
    pub const MODE_3B: Key =    Key::Three;

    pub const EXIT: Key =       Key::Home;
}
