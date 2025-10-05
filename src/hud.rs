use crate::{ eadk::*, input::Mode, config::{ graphics::*, palette::*, strings::* } };
#[cfg(target_os = "none")]
use alloc::format;

fn draw_hud_string(text: &str, bg_color: Color) {
    display::draw_string(
        text,
        Point { x: 10, y: SCREEN_HEIGHT - HUD_HEIGHT + 10 },
        true,
        WHITE,
        bg_color
    );
}

fn draw_help_string(text: &str) {
    display::draw_string(
        text,
        Point {
            x: SCREEN_WIDTH - FB_WIDTH - 2 + 10,
            y: MARGIN_TOP + 10
        },
        false,
        DARK_GREY,
        GREY
    );
}

pub fn draw_hud(mode: Mode, mode_update: bool, help_update: bool, scale: f32, n: usize) {
    let bg_color = match mode {
        Mode::Rotate => RED,
        Mode::Green => GREEN,
        Mode::Translate => BLUE
    };
    
    if mode_update {
        display::push_rect_uniform(
            Rect {
                x: 0,
                y: SCREEN_HEIGHT - HUD_HEIGHT,
                width: SCREEN_WIDTH,
                height: HUD_HEIGHT
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
        Mode::Rotate => ROTATE_NAME,
        Mode::Green => GREEN_NAME,
        Mode::Translate => TRANSLATE_NAME
    };
    let length = text.len() as u16;
    display::draw_string(
        text,
        Point { x: SCREEN_WIDTH - 5 - 7 * length, y: SCREEN_HEIGHT - 15},
        false,
        WHITE,
        bg_color
    );

    if help_update {
        display::push_rect_uniform(
            Rect {
                x: SCREEN_WIDTH - FB_WIDTH - 2,
                y: MARGIN_TOP,
                width: FB_WIDTH,
                height: SCREEN_HEIGHT - MARGIN_TOP - MARGIN_BOTTOM
            },
            GREY
        );
        draw_help_string(
            match mode {
                Mode::Rotate => ROTATE_HELP,
                Mode::Green => GREEN_HELP,
                Mode::Translate => TRANSLATE_HELP
            }
        );
    }
}