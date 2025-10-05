#[cfg(debug_assertions)]
pub mod dev {
    use crate::{
        eadk::*,
        mat::*
    };
    use core::{ ptr, mem };

    const TRI_SIZE: usize = mem::size_of::<Triangle3>();

    pub struct TriangleIter<'a> {
        bytes: &'a [u8],
        index: usize,
    }
    impl<'a> Iterator for TriangleIter<'a> {
        type Item = Triangle3;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index + TRI_SIZE > self.bytes.len() {
                return None;
            }

            let mut tri = Triangle3([Vector3::new(0.0, 0.0, 0.0); 3]);

            unsafe {
                let src = self.bytes.as_ptr().add(self.index);
                let dst = &mut tri as *mut Triangle3 as *mut u8;
                ptr::copy_nonoverlapping(src, dst, TRI_SIZE);
            }
            
            self.index += TRI_SIZE;
            Some(tri)
        }
    }

    pub fn load_tris() -> TriangleIter<'static> {
        let bytes = get_data();
        TriangleIter {
            bytes,
            index: 0,
        }
    }
}
