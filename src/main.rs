#![no_std]
#![no_main]

pub mod eadk;

#[used]
#[cfg(target_os = "none")]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 11] = *b"3D Grapher\0";

#[used]
#[cfg(target_os = "none")]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[cfg(target_os = "none")]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

fn random_u16() -> u16 {
    return eadk::random() as u16;
}

fn random_coordinate() -> u16 {
    return (eadk::random() % 0xFF) as u16;
}

#[no_mangle]
pub fn main() -> isize {
    let mut big_array: [f32; 400] = [0.0; 400];
    for i in 0..100 {
        big_array[i] = 1.0;
        let c = eadk::Color { rgb565: random_u16() };
        let r = eadk::Rect { x: random_coordinate(), y: random_coordinate(), width: random_coordinate(), height: random_coordinate() };
        eadk::display::push_rect_uniform(r, c);
        eadk::display::wait_for_vblank();
    }
    eadk::timing::msleep(5000);
    0
}
