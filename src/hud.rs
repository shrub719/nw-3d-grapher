use crate::{ eadk::*, input::Mode, config::{ graphics::*, palette::* } };
#[cfg(target_os = "none")]
use alloc::format;

fn draw_hud_string(text: &str, bg_color: Color) {
    display::draw_string(
        text,
        Point { x: 10, y: (SCREEN_HEIGHT - HUD_HEIGHT + 10) as u16 },
        true,
        WHITE,
        bg_color
    );
}

pub fn draw_hud(mode: Mode, mode_update: bool, scale: f32, n: usize) {
    let bg_color = match mode {
        Mode::Rotate => RED,
        Mode::Green => GREEN,
        Mode::Translate => BLUE
    };
    
    if mode_update {
        display::push_rect_uniform(
            Rect {
                x: 0,
                y: (SCREEN_HEIGHT - HUD_HEIGHT) as u16,
                width: SCREEN_WIDTH as u16,
                height: HUD_HEIGHT as u16
            },
            bg_color
        );
    }

    draw_hud_string(
        &(match mode {
            Mode::Rotate => format!("scale: {:.2}     ", scale),
            Mode::Translate => format!("tris: {}      ", n),
            Mode::Green => format!("green")
        }),
        bg_color
    );
    
    let text = match mode {
        Mode::Rotate => "VIEW",
        Mode::Green => "TRACE",
        Mode::Translate => "DOMAIN"
    };
    let length = text.len() as u16;
    display::draw_string(
        text,
        Point { x: SCREEN_WIDTH as u16 - 8 * length, y: SCREEN_HEIGHT as u16 - 15},
        false,
        WHITE,
        bg_color
    );
}