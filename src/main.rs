#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![allow(unused)]


// NW
#[allow(unused_imports)]
#[cfg(target_os = "none")]
use cortex_m;

use eadk::heap_size;
#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
#[cfg(target_os = "none")]
static HEAP: Heap = Heap::empty();

#[cfg(target_os = "none")]
extern crate alloc;

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


// Body

pub mod eadk;
mod renderer;
mod mat;
use crate::mat::*;


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
    #[cfg(target_os = "none")]
    {
        let heap_size: usize = heap_size();
        unsafe { HEAP.init(eadk::HEAP_START as usize, heap_size) }
    }
    
    let mut big_array: [f32; 400] = [0.0; 400];
    for i in 0..100 {
        big_array[i] = 1.0;
        let v1 = random_point();
        let v2 = Point2::<u16> { x: v1.x+20, y: v1.y+20};
        let v3 = v2.clone();
        renderer::fill_triangle(v1, v2, v3);
    }
    eadk::timing::msleep(2000);
    0
}
