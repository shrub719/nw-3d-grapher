#[repr(C)]
pub struct Color {
    pub rgb565: u16
}

#[repr(C)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16
}

#[repr(C)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub mod backlight {
    pub fn set_brightness(brightness: u8) {
        unsafe {
            eadk_backlight_set_brightness(brightness);
        }
    }
    pub fn brightness() -> u8 {
        unsafe {
            return eadk_backlight_brightness();
        }
    }

    extern "C" {
        fn eadk_backlight_set_brightness(brightness: u8);
        fn eadk_backlight_brightness() -> u8;
    }

}

pub mod display {
    use super::Rect;
    use super::Color;
    use super::Point;

    #[cfg(target_os = "none")]
    use alloc::ffi::CString;

    #[cfg(not(target_os = "none"))]
    use std::ffi::CString;

    use core::ffi::c_char;

    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    pub fn wait_for_vblank() {
        unsafe {
            eadk_display_wait_for_vblank();
        }
    }

    pub fn draw_string(
        text: &str,
        point: Point,
        large_font: bool,
        text_color: Color,
        background_color: Color,
    ) {
        let c_string =
            CString::new(text).expect("Can't convert str to C_String. Maybe invalid caracter.");
        unsafe {
            eadk_display_draw_string(
                c_string.as_ptr(),
                point,
                large_font,
                text_color,
                background_color,
            )
        }
    }

    extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
        fn eadk_display_push_rect(rect: Rect, color: *const Color);
        fn eadk_display_wait_for_vblank();
        fn eadk_display_draw_string(
            text: *const c_char,
            point: Point,
            large_font: bool,
            text_color: Color,
            background_color: Color,
        );
    }
}

pub mod timing {
    pub fn usleep(us: u32) {
        unsafe {
            eadk_timing_usleep(us);
        }
    }

    pub fn msleep(ms: u32) {
        unsafe {
            eadk_timing_msleep(ms);
        }
    }

    pub fn millis() -> u64 {
        unsafe {
            return eadk_timing_millis();
        }
    }

    extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }
}

pub fn random() -> u32 {
    unsafe {
        return eadk_random()
    }
}

extern "C" {
    fn eadk_random() -> u32;
}

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
use alloc::string::String;

fn write_wrapped(text: &str, limit: usize) {
    let mut line_count = 0;

    let mut line = String::new();
    for i in 0..text.len() {
        line.push(text.as_bytes()[i] as char);

        if line.len() >= limit || text.as_bytes()[i] as char == '\n' || i >= text.len() - 1 {
            display::draw_string(
                line.as_str(),
                Point {
                    x: 10,
                    y: (10 + 20 * line_count) as u16,
                },
                false,
                Color { rgb565: 65503 },
                Color { rgb565: 63488 },
            );
            line.clear();
            line_count += 1;
        }
    }
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    use alloc::format;

    display::push_rect_uniform(
        Rect {
            x: 0,
            y: 0,
            width: 320,
            height: 240,
        },
        Color { rgb565: 63488 },
    ); // Show a red screen

    write_wrapped(format!("{}", panic).as_str(), 42);

    loop {
        
    } // FIXME: Do something better. Exit the app maybe?
}

unsafe extern "C" {
    pub static mut _heap_start: u8;
    pub static mut _heap_end: u8;
}

pub static mut HEAP_START: *mut u8 = core::ptr::addr_of_mut!(_heap_start);
pub static mut HEAP_END: *mut u8 = core::ptr::addr_of_mut!(_heap_end);

pub fn heap_size() -> usize {
    (unsafe { HEAP_END.offset_from(HEAP_START) }) as usize
}
