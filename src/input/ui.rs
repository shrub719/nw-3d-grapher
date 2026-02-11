use crate::{
    input::parser::Expr,
    eadk::{
        display::*,
        Point,
        Color,
        Rect,
        input::*
    },
    constants::{
        controls::*,
        graphics::*,
        palette::*
    }
};
#[cfg(target_os = "none")]
use alloc::string::String;
#[cfg(target_os = "none")]
use alloc::format;

fn write(text: &str) {
    let text_c = format!("{}|", text);
    push_rect_uniform(
        Rect {
            x: 0,
            y: MARGIN_TOP,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT - MARGIN_TOP
        },
        WHITE
    );

    let limit = 30;
    let mut line_count = 0;

    let mut line = String::new();
    for i in 0..text_c.len() {
        line.push(text_c.as_bytes()[i] as char);

        if line.len() >= limit || i >= text_c.len() - 1 {
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

pub fn get_expr() -> Option<Expr> {
    let mut expr = String::new();
    
    let mut keyboard_state = KeyboardState::scan();
    while !keyboard_state.key_down(CONFIRM) {
        keyboard_state = KeyboardState::scan();

        use Event::*;
        let event = event_get(200);
        
        if event == Backspace {
            expr.pop();
            write(&expr);
        } else {
            let c: &str = match event {
                Shift => "z ", Alpha => "y ", Xnt => "x ",
                Zero => "0", One => "1", Two => "2", Three => "3", Four => "4", 
                Five => "5", Six => "6", Seven => "7", Eight => "8", Nine => "9",
                Plus => "+ ", Minus => "- ", Multiplication => "* ", Division => "/ ",
                Power => "^ ",
                Sine => "sin ", Cosine => "cos ", Tangent => "tan ",
                EXE => " ",
                _ => ""
            };

            if c != "" {
                expr.push_str(c);
                write(&expr);
            }
        }

        if keyboard_state.key_down(EXIT) { return None }
    }

    Some(Expr::new(&expr, true).unwrap())
}
