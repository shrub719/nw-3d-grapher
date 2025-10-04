use crate::{ eadk::*, input::Mode, config::graphics::* };

pub fn draw_hud(mode: Mode) {
    let color = match mode {
        Mode::Rotate => Color::from_rgb(0, 255, 255),
        Mode::Translate => Color::from_rgb(255, 0, 0)
    };
    display::push_rect_uniform(
        Rect {
            x: 0,
            y: (SCREEN_HEIGHT - HUD_HEIGHT) as u16,
            width: SCREEN_WIDTH as u16,
            height: HUD_HEIGHT as u16
        },
        color
    );
}