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

fn get_coord(matrix: Matrix4, x: u16, y: u16, z: f32) -> Vector3 {
    let r_vector = RVector3::new(
        x as isize,
        y as isize,
        z as f16
    );

    r_vector * matrix
}

fn march_that_ray(func: fn(f32, f32, f32) -> f32, matrix: Matrix4, x: u16, y: u16) -> Color {
    let z0 = -10.0;
    let z1 = 10.0;
    let dz = (z1 - z0) / MARCH_N as f32;
    let mut z = z0;
    
    let mut c = get_coord(matrix, x, y, z);
    let mut prev_t = func(c.x, c.y, c.z);
    for _ in 0..MARCH_N {
        z += dz;
        c = get_coord(matrix, x, y, z);
        if prev_t * func(c.x, c.y, c.z) < 0.0 {
            break;
        }
        prev_t = func(c.x, c.y, c.z);
    }
    
    let mut value = (-z + 1.0) / 2.0 * 255.0;
    if value > 255.0 { value = 255.0 };
    if value < 0.0 { value = 0.0 };

    Color::from_rgb(0, value as u16, 255)
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

