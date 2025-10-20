use crate::{ 
    eadk::*, 
    grapher::input::Mode, 
    config::{ 
        graphics::*, 
        palette::*, 
        strings::* 
    } 
};
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

fn draw_help_string(text: &str, offset: u16) {
    display::draw_string(
        text,
        Point {
            x: SCREEN_WIDTH - FB_WIDTH - 2 + 10,
            y: MARGIN_TOP + 10 + 15 * offset
        },
        false,
        DARK_GREY,
        GREY
    );
}

pub fn draw_hud(mode: Mode, mode_update: bool, help_on: bool, scale: f32, n: usize) {
    let bg_color = match mode {
        Mode::View => RED,
        Mode::Trace => GREEN,
        Mode::Domain => BLUE
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
            Mode::View => format!("scale: {:.2}     ", scale),
            Mode::Trace => format!("green"),
            Mode::Domain => format!("tris: {}      ", n)
        }),
        bg_color
    );
    
    let text = match mode {
        Mode::View => ROTATE_NAME,
        Mode::Trace => TRACE_NAME,
        Mode::Domain => DOMAIN_NAME
    };
    let length = text.len() as u16;
    display::draw_string(
        text,
        Point { x: SCREEN_WIDTH - 5 - 7 * length, y: SCREEN_HEIGHT - 15},
        false,
        WHITE,
        bg_color
    );

    if help_on {
        let help_lines = match mode {
            Mode::View => ROTATE_HELP,
            Mode::Trace => TRACE_HELP,
            Mode::Domain => DOMAIN_HELP
        };

        display::push_rect_uniform(
            Rect {
                x: SCREEN_WIDTH - FB_WIDTH - 2,
                y: MARGIN_TOP,
                width: FB_WIDTH,
                height: SCREEN_HEIGHT - MARGIN_TOP - MARGIN_BOTTOM
            },
            GREY
        );
        
        let mut i: u16 = 0;
        for text in help_lines {
            draw_help_string(text, i);
            i += 1;
        }
    }
}