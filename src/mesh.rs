use crate::mat::*;
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;
use crate::config::*;

// TODO: add projection matrix
const PROJECTION_MATRIX: Matrix4 = Matrix4 ( [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0]
] );

struct Domain {
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32
}
impl Domain {
    pub fn new() -> Self {
        Domain {
            x0: -10.0,
            y0: -10.0,
            z0: -10.0,
            x1: 10.0,
            y1: 10.0,
            z1: 10.0
        }
    }

    pub fn get_domain_matrix(&self) -> Matrix4 {
        Matrix4::new()
    }
}

struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Rotation {
    pub fn new() -> Self {
        Rotation {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn get_rotation_matrix(&self) -> Matrix4 {
        Matrix4::new()
    }
}

pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub lines: Vec<Line>,
    pub indices: Vec<Vector3>,
    pub transformed_indices: Vec<RVector3>,
    domain: Domain,
    rotation: Rotation
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            // TODO: remember to use with_capacity
            indices: vec![
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 120.0, 0.5),
                Vector3::new(160.0, 0.0, 1.0),
                Vector3::new(160.0, 120.0, 1.0),
            ],
            tris: [
                vec![Triangle ([0, 1, 3]); TEST_N], 
                vec![Triangle ([0, 2, 3]); TEST_N]
            ].concat(),
            transformed_indices: vec![],
            lines: vec![],
            domain: Domain::new(),
            rotation: Rotation::new(),
        }
    }

    pub fn rotate(&mut self, rotation_direction: Vector3) {
        // TODO: do this nicer
        self.rotation.x += rotation_direction.x * ROTATION_SPEED;
        self.rotation.y += rotation_direction.y * ROTATION_SPEED;
        self.rotation.z += rotation_direction.z * ROTATION_SPEED;
    }

    // FIXME: the copying/referencing here is all over the place
    pub fn transform(&mut self) {
        let mut matrix = Matrix4::new();
        matrix *= self.domain.get_domain_matrix();
        matrix *= self.rotation.get_rotation_matrix();
        matrix *= PROJECTION_MATRIX;

        self.transformed_indices.clear();
        for vertex in &self.indices {
            self.transformed_indices.push(RVector3::from_vector3(vertex * &matrix));
        }
    }
}

// TODO: add normals to triangle struct? for lighting
#[derive(Clone, Copy)]
pub struct Triangle(pub [usize; 3]);

#[derive(Clone, Copy)]
pub struct Line(pub [usize; 2]);
