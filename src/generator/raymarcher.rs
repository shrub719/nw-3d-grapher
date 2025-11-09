use crate::{
    eadk::*,
    grapher::{
        mat::*
    },
    constants::{
        limits::*,
        graphics::*
    }
};

pub fn march_that_ray(func: fn(f32, f32, f32) -> f32, matrix: Matrix4, x: u16, y: u16) -> Color {
    BG
}

pub fn generate_screen(func: fn(f32, f32, f32) -> f32, matrix: Matrix4) {
    let mut row_buffer: [Color; SCREEN_WIDTH_SIZE] = [BG; SCREEN_WIDTH_SIZE];

    for y in MARGIN_TOP..MARGIN_TOP+FRAME_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            row_buffer[x as usize] = march_that_ray(func, matrix, x, y);
        }

        display::push_rect(
            Rect {
                x: 0, 
                y,
                width: SCREEN_WIDTH,
                height: 1
            },
            &row_buffer
        );
    }
}

