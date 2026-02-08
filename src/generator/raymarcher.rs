use crate::{
    eadk::{
        *,
        input::*
    },
    grapher::{
        mat::*
    },
    constants::{
        limits::*,
        graphics::*,
        controls::*
    },
    menu::parser::Expr
};
#[cfg(target_os = "none")]
use alloc::format;

fn get_coord(matrix: Matrix4, x: u16, y: u16, z: f32) -> Vector3 {
    let r_vector = RVector3::new(
        x as isize,
        y as isize,
        z as f16
    );

    r_vector * matrix
}

fn march_that_ray(expr: &Expr, matrix: Matrix4, n: usize, x: u16, y: u16) -> Color {
    let z0 = -1.0;
    let z1 = 1.0;
    let dz = (z1 - z0) / n as f32;
    let mut z = z0;
    
    let mut c = get_coord(matrix, x, y, z);
    let mut prev_t = expr.eval(c.x, c.y, c.z).unwrap();
    let mut i = 0;
    while i < n {
        z += dz;
        c = get_coord(matrix, x, y, z);
        if prev_t * expr.eval(c.x, c.y, c.z).unwrap() < 0.0 {
            break;
        }
        prev_t = expr.eval(c.x, c.y, c.z).unwrap();
        i += 1;
    }
    
    let mut value = (-z + 1.0) / 2.0 * 255.0;
    if value > 255.0 { value = 255.0 };
    if value < 0.0 { value = 0.0 };

    if i == n { BG } else { Color::from_rgb(0, value as u16, 255) }
}

pub fn generate_screen(expr: &Expr, matrix: Matrix4) {
    let mut row_buffer: [Color; SCREEN_WIDTH_SIZE] = [BG; SCREEN_WIDTH_SIZE];
    let mut keyboard_state: KeyboardState;
    let mut n = MARCH_N;

    let mut prev_time = timing::millis();
    for y in MARGIN_TOP..MARGIN_TOP+FRAME_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            row_buffer[x as usize] = march_that_ray(expr, matrix, n, x, y);
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
        
        let current_time = timing::millis();
        if (current_time - prev_time) > MAX_ROW_TIME {
            n = (n as f32 * 0.9) as usize;
        }
        prev_time = current_time;

        let progress = (y - MARGIN_TOP) as f32 / (FRAME_HEIGHT) as f32;
        header_info(&format!(
            "{:.0}%         ", progress * 100.0
        ));
        #[cfg(debug_assertions)]
        header_info(&format!(
            "{}             ", n
        ));

        keyboard_state = KeyboardState::scan();
        if keyboard_state.key_down(BACK) {
            break;
        }
    }
}

