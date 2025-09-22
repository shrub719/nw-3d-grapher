use crate::mat::*;
use crate::trig::*;
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;
use crate::config::*;

// TODO: add projection matrix
const PROJECTION_MATRIX: Matrix4 = Matrix4 ( [
    [1.0, 0.0, 0.0, 160.0],
    [0.0, 1.0, 0.0, 120.0],
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
        Matrix4 ( [
            [1.0, 0.0, 0.0, -160.0],
            [0.0, 1.0, 0.0, -120.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ] )
        // TODO
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
        let mut matrix = Matrix4::new();

        let sin_x = sin(self.x);
        let cos_x = cos(self.x);
        let sin_y = sin(self.y);
        let cos_y = cos(self.y);
        let sin_z = sin(self.z);
        let cos_z = cos(self.z);

        matrix *= Matrix4 ( [
            [1.0   , 0.0   , 0.0   , 0.0],
            [0.0   , cos_x , -sin_x, 0.0],
            [0.0   , sin_x , cos_x , 0.0],
            [0.0   , 0.0   , 0.0   , 1.0]
        ] );
        matrix *= Matrix4 ( [
            [cos_y , 0.0   , sin_y , 0.0],
            [0.0   , 1.0   , 0.0   , 0.0],
            [-sin_y, 0.0   , cos_y , 0.0],
            [0.0   , 0.0   , 0.0   , 1.0]
        ] );
        matrix *= Matrix4 ( [
            [cos_z , -sin_z, 0.0   , 0.0],
            [sin_z , cos_z , 0.0   , 0.0],
            [0.0   , 0.0   , 1.0   , 0.0],
            [0.0   , 0.0   , 0.0   , 1.0]
        ] );

        matrix
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
                Vector3::new(0.0, 120.0, 0.0),
                Vector3::new(160.0, 0.0, 0.0),
                Vector3::new(160.0, 120.0, 0.0),
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

    pub fn update_rotation(&mut self, rotation_direction: Vector3) {
        // TODO: do this nicer
        self.rotation.x += rotation_direction.x * ROTATION_SPEED;
        self.rotation.y += rotation_direction.y * ROTATION_SPEED;
        self.rotation.z += rotation_direction.z * ROTATION_SPEED;
    }

    pub fn transform(&mut self) {
        let mut matrix = PROJECTION_MATRIX;
        matrix *= self.rotation.get_rotation_matrix();
        matrix *= self.domain.get_domain_matrix();

        self.transformed_indices.clear();
        for vertex in &self.indices {
            self.transformed_indices.push(RVector3::from_vector3(*vertex * matrix));
        }
    }
}

// TODO: add normals to triangle struct? for lighting
#[derive(Clone, Copy)]
pub struct Triangle(pub [usize; 3]);

#[derive(Clone, Copy)]
pub struct Line(pub [usize; 2]);
