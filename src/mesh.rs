use crate::mat::*;
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub lines: Vec<Line>,
    pub indices: Vec<Vector3>,
    pub transformed_indices: Vec<RVector3>
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            tris: Vec::with_capacity(50),
            lines: Vec::with_capacity(50),
            indices: Vec::with_capacity(50),
            transformed_indices: Vec::with_capacity(50)
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

// TODO: add normals to triangle struct? for lighting
#[derive(Clone, Copy)]
pub struct Triangle(pub [usize; 3]);

#[derive(Clone, Copy)]
pub struct Line(pub [usize; 2]);
