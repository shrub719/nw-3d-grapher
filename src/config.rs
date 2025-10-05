pub mod settings {
    pub const ROTATION_SPEED: f32 = 1.5;
    pub const SCALE_SPEED: f32 = 0.5;
}

pub mod test {
    pub const TEST_N: usize = 50;
}

pub mod palette {
    use crate::eadk::Color;

    pub const ORANGE: Color = Color::from_rgb(255, 183, 52);
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);
    pub const GREY: Color = Color::from_rgb(230, 230, 230);
    pub const DARK_GREY: Color = Color::from_rgb(75, 75, 75);

    pub const RED: Color = Color::from_rgb(255, 90, 75);
    pub const GREEN: Color = Color::from_rgb(75, 200, 90);
    pub const BLUE: Color = Color::from_rgb(75, 90, 255);
}

pub mod graphics {
    // TODO: also make u16 constants
    pub const SCREEN_WIDTH: u16 = 320;
    pub const SCREEN_HEIGHT: u16 = 240;

    pub const HUD_HEIGHT: u16 = 36;

    pub const MARGIN_TOP: u16 = 18;
    pub const MARGIN_BOTTOM: u16 = HUD_HEIGHT;

    // 2 is possible with margins and very few triangles... i kinda wanna try stretch it further
    pub const FB_TILE: u16 = 3;

    pub const FB_WIDTH: u16 = SCREEN_WIDTH / FB_TILE;
    pub const FB_HEIGHT: u16 = (SCREEN_HEIGHT - (MARGIN_TOP + MARGIN_BOTTOM)) / FB_TILE;
    pub const FB_WIDTH_SIZE: usize = FB_WIDTH as usize;
    pub const FB_HEIGHT_SIZE: usize = FB_HEIGHT as usize;

    use crate::eadk;
    pub const BG: eadk::Color = eadk::Color::from_rgb(255, 255, 255);
}

pub mod limits {
    pub const MAX_TRIS: usize = 2000;
    // pub const MAX_LINES: usize = 20;  // TODO: add lines
}

pub mod strings {
    pub const ROTATE_NAME: &str = "VIEW";
    pub const GREEN_NAME: &str = "TRACE";
    pub const TRANS_NAME: &str = "DOMAIN";

    type Help<'a> = [&'a str; 3];
    pub const ROTATE_HELP: Help = [
        "D-pad: Rotate",  // max line length
        "+/-: Scale",
        ""
    ];
    pub const GREEN_HELP: Help = [
        "Green :)",
        "Does nothing",
        ""
    ];
    pub const TRANS_HELP: Help = [
        "Up: Randomise",
        "+/-: Change n",
        "3: Secret"
    ];
}
