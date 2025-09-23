use crate::mat::*;
use crate::trig::*;
use crate::eadk::info;
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(target_os = "none")]
use alloc::format;
use crate::config::*;

const PROJECTION_MATRIX: Matrix4 = Matrix4 ( [
    [120.0, 0.0, 0.0, 160.0],
    [0.0, 120.0, 0.0, 120.0],
    [0.0, 0.0, 1.0, 0.0],     // TODO: what's going on with the z coord
    [0.0, 0.0, 0.0, 1.0]
] );

struct Domain {
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
    pub matrix: Matrix4
}
impl Domain {
    pub fn new() -> Self {
        Domain {
            x0: -10.0,
            y0: -10.0,
            z0: -10.0,
            x1: 10.0,
            y1: 10.0,
            z1: 10.0,
            matrix: Matrix4::new()
        }
    }

    pub fn update_matrix(&mut self) {
        let dx = self.x1 - self.x0;
        let dy = self.y1 - self.y0;
        let dz = self.z1 - self.z0;

        let x_scale = 2.0 / dx;
        let y_scale = 2.0 / dy;
        let z_scale = 2.0 / dz;

        let x_trans = -self.x0 - dx/2.0;
        let y_trans = -self.y0 - dy/2.0;
        let z_trans = -self.z0 - dz/2.0;

        self.matrix = Matrix4 ( [
            [x_scale, 0.0    , 0.0    , x_trans*x_scale],
            [0.0    , y_scale, 0.0    , y_trans*y_scale],
            [0.0    , 0.0    , z_scale, z_trans*z_scale],
            [0.0    , 0.0    , 0.0    , 1.0            ]
        ] )
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
                Vector3::new(-3.0, -3.0, 10.0),
                Vector3::new(-3.0, 3.0, 10.0),
                Vector3::new(3.0, -3.0, 10.0),
                Vector3::new(3.0, 3.0, 10.0),
                Vector3::new(-3.0, -3.0, -10.0),
                Vector3::new(-3.0, 3.0, -10.0),
                Vector3::new(3.0, -3.0, -10.0),
                Vector3::new(3.0, 3.0, -10.0),
            ],
            tris: vec![
                // square1
                [Triangle ([0, 1, 3]); test::TEST_N], 
                [Triangle ([0, 2, 3]); test::TEST_N],

                // square2
                [Triangle ([4, 5, 7]); test::TEST_N], 
                [Triangle ([4, 6, 7]); test::TEST_N],

                // side1
                [Triangle ([0, 1, 4]); test::TEST_N], 
                [Triangle ([1, 4, 5]); test::TEST_N],
                // FIXME: index oob: 604057136
            ].concat(),
            transformed_indices: vec![],
            lines: vec![],
            domain: Domain::new(),
            rotation: Rotation::new(),
        }
    }

    pub fn update_domain(&mut self) {
        // DEBUG
        info(&format!("{} tris {} vx", self.tris.len(), self.indices.len()));
        self.domain.update_matrix();
    }

    pub fn update_rotation(&mut self, rotation_direction: Vector3) {
        self.rotation.x += rotation_direction.x * settings::ROTATION_SPEED;
        self.rotation.y += rotation_direction.y * settings::ROTATION_SPEED;
        self.rotation.z += rotation_direction.z * settings::ROTATION_SPEED;
    }

    pub fn transform(&mut self) {
        let mut matrix = PROJECTION_MATRIX;
        matrix *= self.rotation.get_rotation_matrix();
        matrix *= self.domain.matrix;

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
