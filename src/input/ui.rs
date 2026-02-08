use crate::{
    input::parser::Expr,
    eadk::{
        display::draw_string,
        Point,
        Color,
        input::*
    },
    constants::controls::*
};
#[cfg(target_os = "none")]
use alloc::string::String;

fn write(text: &str) {
    let limit = 30;
    let mut line_count = 0;

    let mut line = String::new();
    for i in 0..text.len() {
        line.push(text.as_bytes()[i] as char);

        if line.len() >= limit || i >= text.len() - 1 {
            draw_string(
                line.as_str(),
                Point {
                    x: 10,
                    y: (25 + 20 * line_count) as u16
                },
                true,
                Color::from_rgb(0, 0, 0),
                Color::from_rgb(255, 255, 255)
            );
            line.clear();
            line_count += 1;
        }
    }
}

pub fn get_expr() -> Expr {
    let expr: &str = "x 2 ^ y 2 ^ + z 2 ^ + 4 x * sin + 4 y * sin + 4 z * sin + 1.11 -";
    write(expr);
    
    let mut keyboard_state = KeyboardState::scan();
    while !keyboard_state.key_down(CONFIRM) {
        keyboard_state = KeyboardState::scan();
    }

    Expr::new(expr, false).unwrap()
}
