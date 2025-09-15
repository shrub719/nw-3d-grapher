use crate::mat::*;
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub indices: Vec<Vector3>,
    pub transformed_indices: Vec<RVector3>
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            tris: vec![],
            indices: vec![],
            transformed_indices: vec![]
        }
    }

    // FIXME: the copying/referencing here is all over the place
    pub fn transform(&mut self, matrix: &Matrix3) {
        self.transformed_indices.clear();
        for vertex in &self.indices {
            self.transformed_indices.push(RVector3::from_vector3(vertex * matrix));
        }
    }
}
