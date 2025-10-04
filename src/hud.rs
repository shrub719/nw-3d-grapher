use crate::{ eadk::*, input::Mode, config::graphics::* };
#[cfg(target_os = "none")]
use alloc::format;

fn draw_hud_string(text: &str, bg_color: Color) {
    display::draw_string(
        text,
        Point { x: 10, y: (SCREEN_HEIGHT - HUD_HEIGHT + 10) as u16 },
        true,
        Color::from_rgb(255, 255, 255),
        bg_color
    );
}

pub fn draw_hud(mode: Mode, scale: f32, n: usize) {
    let bg_color = match mode {
        Mode::Rotate => Color::from_rgb(0, 0, 255),
        Mode::Translate => Color::from_rgb(255, 0, 0)
    };
    display::push_rect_uniform(
        Rect {
            x: 0,
            y: (SCREEN_HEIGHT - HUD_HEIGHT) as u16,
            width: SCREEN_WIDTH as u16,
            height: HUD_HEIGHT as u16
        },
        bg_color
    );

    draw_hud_string(
        &(match mode {
            Mode::Rotate => format!("scale: {:.2}", scale),
            Mode::Translate => format!("tris: {}", n)
        }),
        bg_color
    );
}