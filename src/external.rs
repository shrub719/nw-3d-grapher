use crate::{
    eadk::get_data,
    mat::*
};

pub fn load_tris() -> &'static [Triangle3] {
    let bytes = get_data();

    unsafe {
        core::slice::from_raw_parts(
            bytes.as_ptr() as *const Triangle3,
            bytes.len() / core::mem::size_of::<Triangle3>(),
        )
    }
}