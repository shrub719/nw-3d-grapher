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

    pub const FB_TILE: usize = 3;

    pub const FB_WIDTH: usize = SCREEN_WIDTH / FB_TILE;
    pub const FB_HEIGHT: usize = SCREEN_HEIGHT / FB_TILE;
}

pub mod limits {
    pub const MAX_TRIS: usize = 500;
}
