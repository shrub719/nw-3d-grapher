#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![allow(unused)]
#![feature(f16)]

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
#[unsafe(link_section = ".rodata.eadk_app_name")]
pub static EADK_APP_NAME: [u8; 11] = *b"3D Grapher\0";

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_api_level")]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
pub static EADK_APP_ICON: [u8; 2769] = *include_bytes!("../target/icon.nwi");

pub mod eadk;
mod config;
mod renderer;
mod mat;
mod mesh;
use crate::mat::*;
use crate::mesh::*;
use crate::config::*;
use crate::renderer::*;
#[cfg(target_os = "none")]
use alloc::vec;

fn random_coordinate() -> u16 {
    return (eadk::random() % 0xFF) as u16;
}


#[unsafe(no_mangle)]
pub fn main() -> isize {
    #[cfg(target_os = "none")]
    {
        let heap_size: usize = heap_size();
        unsafe { HEAP.init(eadk::HEAP_START as usize, heap_size) }
    }

    // usually this would be recalculated for every change in function/domain
    let mut mesh = Mesh {
        indices: vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 120.0, 0.5),
            Vector3::new(160.0, 0.0, 1.0),
            Vector3::new(160.0, 120.0, 1.0),
        ],
        tris: vec![Triangle ([0, 1, 3]), Triangle ([0, 2, 3])],
        transformed_indices: vec![],
        lines: vec![]
    };

    let scale = 2.0;
    let matrix = Matrix3 ( [
        [scale, 0.0, 0.0],
        [0.0, scale, 0.0],
        [0.0, 0.0, 1.0]
    ] );

    // main loop - runs every frame
    let mut keyboard_state: eadk::input::KeyboardState = eadk::input::KeyboardState::scan();
    while !keyboard_state.key_down(eadk::input::Key::Back) {
        let color = eadk::Color::from_rgb(eadk::random() as u16, eadk::random() as u16, eadk::random() as u16);
        mesh.transform(&matrix);
        renderer::draw_screen(&mesh, color);

        keyboard_state = eadk::input::KeyboardState::scan();
        while !(keyboard_state.key_down(eadk::input::Key::Ok) || keyboard_state.key_down(eadk::input::Key::Back)) {
            keyboard_state = eadk::input::KeyboardState::scan();
            eadk::timing::msleep(50);
        }
    }
    0
}
