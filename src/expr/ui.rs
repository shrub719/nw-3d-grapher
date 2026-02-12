use crate::{
    expr::parser::Expr,
    eadk::{
        display::*,
        Point,
        Color,
        input::*
    },
    constants::controls::*
};
#[cfg(target_os = "none")]
use alloc::string::String;
#[cfg(target_os = "none")]
use alloc::format;

fn write(text: &str, is_implicit: bool) {
    let pre = if is_implicit { "0 =" } else { "z =" };
    let text_c = format!("{} {}| ", pre, text);

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
    let mut is_implicit = true;
    write(&expr, is_implicit);
    
    loop {
        use Event::*;
        let event = event_get(100);
        
        if event == Backspace {
            expr.pop();
            write(&expr, is_implicit);
        } else if event == Toolbox {
            is_implicit = !is_implicit;
            write(&expr, is_implicit);
        } else if event == OK {
            break;
        } else {
            let c: &str = match event {
                LowerZ => "z ", LowerY => "y ", Xnt => "x ", LowerX => "x ",
                Zero => "0", One => "1", Two => "2", Three => "3", Four => "4", 
                Five => "5", Six => "6", Seven => "7", Eight => "8", Nine => "9",
                Plus => "+ ", Minus => "- ", Multiplication => "* ", Division => "/ ",
                Power => "^ ", Square => "2 ^ ",
                Sine => "sin ", Cosine => "cos ", Tangent => "tan ",
                EXE => " ", Space => " ",
                _ => ""
            };

            if c != "" {
                expr.push_str(c);
                write(&expr, is_implicit);
            }
        }

        if KeyboardState::scan().key_down(EXIT) { return None }
    }

    Some(Expr::new(&expr, is_implicit).unwrap())
}
