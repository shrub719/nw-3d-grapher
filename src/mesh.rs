use crate::trig::*;
use crate::mat::*;
use crate::eadk::info;
use crate::eadk::random;
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;
use crate::config::*;

const PROJECTION_MATRIX: Matrix4 = Matrix4 ( [
    [120.0, 0.0, 0.0, 160.0],
    [0.0, 120.0, 0.0, 120.0],
    [0.0, 0.0, 1.0, 0.0],     // TODO: what's going on with the z coord
    [0.0, 0.0, 0.0, 1.0]
] );

fn get_scale_matrix(scale: f32) -> Matrix4 {
    Matrix4 ([
        [scale, 0.0  , 0.0  , 0.0],
        [0.0  , scale, 0.0  , 0.0],
        [0.0  , 0.0  , scale, 0.0],
        [0.0  , 0.0  , 0.0  , 1.0]
    ])
}

fn random_coord() -> f32 {
    (random() as u16 as f32) / (u16::MAX as f32) * 20.0 - 10.0
}

fn random_point() -> Vector3 {
    Vector3::new(random_coord(), random_coord(), random_coord())
}

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

pub struct Mesh {
    pub tris: Vec<Triangle3>,
    pub transformed_tris: Vec<RTriangle3>,
    // pub lines: Vec<Line>,
    domain: Domain,
    rotation: Quaternion,
    n_tris: usize,
    scale: f32
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            tris: Vec::with_capacity(limits::MAX_TRIS), // TODO: DON'T let these be reallocated lmao
            transformed_tris: Vec::with_capacity(limits::MAX_TRIS),
            // lines:  Vec::with_capacity(limits::MAX_LINES), // TODO: transform lines
            domain: Domain::new(),
            rotation: Quaternion::default(),
            n_tris: test::TEST_N,
            scale: 1.0
        }
    }

    pub fn generate_mesh(&mut self, n_change: isize) {
        let mut n_i = self.n_tris as isize;
        n_i += n_change;
        if n_i < 1 { n_i = 1}
        self.n_tris = n_i as usize;

        self.tris.clear();
        for i in 0..self.n_tris {
            self.tris.push(
                Triangle3([
                    random_point(), random_point(), random_point()
                ])
            );
        }
    }

    pub fn update_domain(&mut self) {
        // DEBUG
        self.domain.update_matrix();
    }

    pub fn update_rotation(&mut self, rotation_direction: Vector3, delta_time: f32) {
        let rotation_speed = settings::ROTATION_SPEED * delta_time;
        let x = rotation_direction.x * rotation_speed / 2.0;
        let y = rotation_direction.y * rotation_speed / 2.0;
        let z = rotation_direction.z * rotation_speed / 2.0;

        let (cx, sx) = (cos(x), sin(x));
        let (cy, sy) = (cos(y), sin(y));
        let (cz, sz) = (cos(z), sin(z));

        let q = Quaternion::new(
            cx*cy*cz + sx*sy*sz, 
            sx*cy*cz + cx*sy*sz, 
            cx*sy*cz + sx*cy*sz, 
            cx*cy*sz + sx*sy*cz
        );
        self.rotation = q * self.rotation;
    }

    pub fn update_scale(&mut self, scale_change: f32, delta_time: f32) {
        self.scale += scale_change * delta_time;
        if self.scale < 0.0 { self.scale = 0.0 }
    }

    pub fn transform(&mut self) {
        let mut matrix = PROJECTION_MATRIX;
        // TODO: can you use quaternions to do the rotation without the translation?
        matrix *= self.rotation.get_rotation_matrix();
        matrix *= self.domain.matrix;
        matrix *= get_scale_matrix(self.scale);

        self.transformed_tris.clear();
        for tri in &self.tris {
            self.transformed_tris.push(*tri * matrix);
        }
    }
}
