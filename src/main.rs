#![no_std]
#![no_main]
#![allow(unused)]

pub mod eadk;
mod renderer;
mod mat;
use crate::mat::*;

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
pub static EADK_APP_ICON: [u8; 2769] = *include_bytes!("../target/icon.nwi");

fn random_u16() -> u16 {
    return eadk::random() as u16;
}

fn random_point() -> Point2<u16> {
    return Point2 { x: random_u16(), y: random_u16() };
}

fn random_coordinate() -> u16 {
    return (eadk::random() % 0xFF) as u16;
}

#[no_mangle]
pub fn main() -> isize {
    let mut big_array: [f32; 400] = [0.0; 400];
    for i in 0..100 {
        big_array[i] = 1.0;
        renderer::fill_triangle(random_point(), random_point(), random_point());
    }
    eadk::timing::msleep(5000);
    0
}
