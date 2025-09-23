pub mod settings {
    pub const ROTATION_SPEED: f32 = 0.2;
}

pub mod test {
    pub const TEST_N: usize = 5;
    pub const TEST_SCALE: f32 = 2.0;
}

pub mod graphics {
    pub const SCREEN_WIDTH: usize = 320;
    pub const SCREEN_HEIGHT: usize = 240;
    pub const MARGIN_TOP: usize = 18;
    pub const MARGIN_SIDE: usize = 18;

    pub const FB_TILE: usize = 3;

    pub const FB_WIDTH: usize = (SCREEN_WIDTH - 2 * MARGIN_SIDE) / FB_TILE;
    pub const FB_HEIGHT: usize = (SCREEN_HEIGHT - MARGIN_TOP) / FB_TILE;
}

pub mod limits {
    pub const MAX_TRIS: usize = 500;
}
