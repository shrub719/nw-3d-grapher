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
mod config;
mod renderer;
mod mat;
use crate::mat::*;
#[cfg(target_os = "none")]
use alloc::vec;
use crate::config::*;
use crate::renderer::*;

fn random_u16() -> u16 {
    return eadk::random() as u16;
}

fn random_coordinate() -> u16 {
    return (eadk::random() % 0xFF) as u16;
}

fn random_point() -> RVector2 {
    return RVector2 { x: random_coordinate() as isize, y: random_coordinate() as isize };
}

#[no_mangle]
pub fn main() -> isize {
    #[cfg(target_os = "none")]
    {
        let heap_size: usize = heap_size();
        unsafe { HEAP.init(eadk::HEAP_START as usize, heap_size) }
    }

    let mesh = Mesh3 {
        indices: vec![
            Vector3::new(random_coordinate() as f32, 0.0, 50.0),
            Vector3::new(0.4, 100.0, 5.09),
            Vector3::new(100.0, random_coordinate() as f32, -1.0)
        ],
        tris: vec![Triangle3 ([0, 1, 2]); TEST_N]
    };

    let mut keyboard_state: eadk::input::KeyboardState = eadk::input::KeyboardState::scan();
    while !keyboard_state.key_down(eadk::input::Key::Back) {
        renderer::draw_screen(&mesh);

        keyboard_state = eadk::input::KeyboardState::scan();
        while !(keyboard_state.key_down(eadk::input::Key::Ok) || keyboard_state.key_down(eadk::input::Key::Back)) {
            keyboard_state = eadk::input::KeyboardState::scan();
            eadk::timing::msleep(50);
        }
    }
    0
}
