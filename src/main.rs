#![cfg_attr(target_os = "none", no_std)]
#![no_main]
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
pub mod external;
mod config;
mod trig;
mod mat;
mod renderer;
mod mesh;
mod input;
mod hud;
mod timer;
mod grapher;
use grapher::Grapher;

#[unsafe(no_mangle)]
pub fn main() -> isize {
    #[cfg(target_os = "none")]
    {
        let heap_size: usize = heap_size();
        unsafe { HEAP.init(eadk::HEAP_START as usize, heap_size) }
    }

    let mut grapher = Grapher::new();
    grapher.main_loop();

    0
}
