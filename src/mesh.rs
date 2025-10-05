use crate::mat::*;
use crate::eadk::random;
#[cfg(target_os = "none")]
use alloc::vec::Vec;
use crate::config::*;
#[cfg(feature = "obj")]
use crate::external::obj::load_tris;

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
    pub scale: f32
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            tris: Vec::with_capacity(limits::MAX_TRIS), // TODO: DON'T let these be reallocated lmao
            // TODO: maybe use arrays instead cause these don't seem to be properly initialised
            // (attempt to subtract with overflow errors in random places when n > ~824)
            transformed_tris: Vec::with_capacity(limits::MAX_TRIS),
            // lines:  Vec::with_capacity(limits::MAX_LINES), // TODO: transform lines
            domain: Domain::new(),
            rotation: Quaternion::default(),
            scale: 0.5
        }
    }

    pub fn generate_mesh(&mut self, n_change: isize) {
        if n_change == 0 {
            self.tris.clear();
            for _ in 0..test::TEST_N {
                self.tris.push(
                    Triangle3([
                        random_point(), random_point(), random_point()
                    ])
                );
            }
        }
        else if n_change > 0 {
            self.tris.push(
                Triangle3([
                    random_point(), random_point(), random_point()
                ])
            );
        } else if n_change < 0 && self.tris.len() > 1 {
            self.tris.pop();
        }
    }

    pub fn load_mesh_from_file(&mut self) {
        self.tris.clear();
        #[cfg(feature = "obj")]
        for tri in load_tris() {
            self.tris.push(tri);
        }
    }

    pub fn update_domain(&mut self) {
        self.domain.update_matrix();
    }

    pub fn update_rotation(&mut self, rotation_direction: Vector3, delta_time: f32) {
        let rotation_speed = settings::ROTATION_SPEED * delta_time;
        let x = rotation_direction.x * rotation_speed;
        let y = rotation_direction.y * rotation_speed;
        let z = rotation_direction.z * rotation_speed;
        
        self.rotation = Quaternion::from_angles(x, y, z) * self.rotation;
    }

    pub fn update_scale(&mut self, scale_change: f32, delta_time: f32) {
        self.scale += settings::SCALE_SPEED * scale_change * delta_time;
        if self.scale < 0.0 { self.scale = 0.0 }
    }

    pub fn transform(&mut self) {
        let mut matrix = PROJECTION_MATRIX;
        // TODO: can you use quaternions to do the rotation without the translation?
        matrix *= self.rotation.get_rotation_matrix();
        matrix *= get_scale_matrix(self.scale);
        matrix *= self.domain.matrix;

        self.transformed_tris.clear();
        for tri in &self.tris {
            self.transformed_tris.push(*tri * matrix);
        }
    }
}
