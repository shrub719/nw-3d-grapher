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
            indices: vec![
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 120.0, 0.5),
                Vector3::new(160.0, 0.0, 1.0),
                Vector3::new(160.0, 120.0, 1.0),
            ],
            tris: vec![
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
                Triangle ([0, 1, 3]), 
                Triangle ([0, 2, 3]),
            ],
            transformed_indices: vec![],
            lines: vec![]
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
