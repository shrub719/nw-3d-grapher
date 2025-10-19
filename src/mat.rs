use core::ops::{ SubAssign, Mul, MulAssign };
use crate::trig::*;
use crate::eadk::Color;

#[derive(Clone, Copy, Debug)]
pub struct RVector3 {
    pub x: isize,
    pub y: isize,
    pub z: f32
}
impl RVector3 {
    pub fn new(x: isize, y: isize, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_vector3 (vector3: Vector3) -> Self {
        Self {
           x: vector3.x as isize,
           y: vector3.y as isize,
           z: vector3.z
        }
    }
}
// impl AddAssign for RVector3 {
//     fn add_assign(&mut self, other: Self) {
//         self.x += other.x;
//         self.y += other.y;
//         self.z += other.z;
//     }
// }
impl SubAssign for RVector3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
// impl Index<usize> for Vector3 {
//     type Output = f32;
//     fn index(&self, i: usize) -> &f32 {
//         match i {
//             0 => &self.x,
//             1 => &self.y,
//             2 => &self.z,
//             _ => panic!("Index out of range for Vector3"),
//         }
//     }
// }
// impl IndexMut<usize> for Vector3 {
//     fn index_mut(&mut self, i: usize) -> &mut f32 {
//         match i {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             2 => &mut self.z,
//             _ => panic!("Index out of range for Vector3"),
//         }
//     }
// }
// technically the wrong order but idc
// impl Mul<&Matrix3> for &Vector3 {
//     type Output = Vector3;

//     fn mul(self, matrix: &Matrix3) -> Vector3 {
//         let mut result = Vector3::new(0.0, 0.0, 0.0);
//         for i in 0..3 {
//             let mut sum: f32 = 0.0;
//             for j in 0..3 {
//                 sum += matrix.0[i][j] * self[j];
//             }
//             result[i] = sum;
//         }
//         result
//     }
// }
impl Mul<Matrix4> for Vector3 {
    type Output = Vector3;

    fn mul(self, matrix: Matrix4) -> Vector3 {
        let self_4 = [self.x, self.y, self.z, 1.0];
        let mut result = [0.0; 4];
        for i in 0..4 {
            let mut sum: f32 = 0.0;
            for j in 0..4 {
                sum += matrix.0[i][j] * self_4[j];
            }
            result[i] = sum;
        }
        Vector3 {
            x: result[0],
            y: result[1],
            z: result[2]
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix4(pub [[f32; 4]; 4]);
impl Matrix4 {
    pub fn new() -> Self {
        Matrix4 ( [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ] )
    }
}
impl MulAssign for Matrix4 {
    fn mul_assign(&mut self, other: Matrix4) {
        let self_copy = *self;
        for i in 0..4 {
            for j in 0..4 {
                let mut sum: f32 = 0.0;
                for k in 0..4 {
                    // OTHER * SELF so that other transformation applies after self
                    sum += other.0[k][j] * self_copy.0[i][k];
                }
                self.0[i][j] = sum;
            }
        }
    }
}


// void matrix_mul(float (&multiplier)[3][3], float (&matrix)[3][3]) {
//     float result[3][3];
//     for (int i = 0; i < 3; i++) {
//         for (int j = 0; j < 3; j++) {
//             float sum = 0.0f;
//             for (int k = 0; k < 3; k++) {
//                 sum += multiplier[i][k] * matrix[k][j];
//             }
//             result[i][j] = sum;
//         }
//     }
//
//     for (int i = 0; i < 3; i ++) {
//         for (int j = 0; j < 3; j++) {
//             matrix[i][j] = result[i][j];
//         }
//     }
// }

// again, wrong order... kinda? *= isn't a mathematical operator
// #[derive(Debug, Clone, Copy)]
// pub struct Matrix3(pub [[f32; 3]; 3]);
// impl MulAssign for Matrix3 {
//     fn mul_assign(&mut self, other: Matrix3) {
//         let self_copy = *self;
//         for i in 0..3 {
//             for j in 0..3 {
//                 let mut sum: f32 = 0.0;
//                 for k in 0..3 {
//                     // OTHER * SELF so that other transformation applies after self
//                     sum += other.0[k][j] * self_copy.0[i][k];
//                 }
//                 self.0[i][j] = sum;
//             }
//         }
//     }
// }

// TODO: add normals to triangle struct? for lighting
#[derive(Clone, Copy)]
pub struct Triangle3(pub [Vector3; 3]);
impl Mul<Matrix4> for Triangle3 {
    type Output = RTriangle3;

    fn mul(self, matrix: Matrix4) -> RTriangle3 {
        let mut result = RTriangle3::new();
        let mut index: usize = 0;
        for vertex in self.0 {
            result.v[index] = RVector3::from_vector3(vertex * matrix);
            index += 1;
        }
        result
    }
}

#[derive(Clone, Copy)]
pub struct Line3(pub [Vector3; 2]);
impl Mul<Matrix4> for Line3 {
    type Output = RLine3;

    fn mul(self, matrix: Matrix4) -> RLine3 {
        let mut result = RLine3 ( [RVector3::new(0, 0, 0.0); 2]);
        let mut index: usize = 0;
        for vertex in self.0 {
            result.0[index] = RVector3::from_vector3(vertex * matrix);
            index += 1;
        }
        result
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTriangle3 {
    pub v: [RVector3; 3],
    pub color: Color
}
impl SubAssign<RVector3> for RTriangle3 {
    fn sub_assign(&mut self, vector: RVector3) {
        for mut vertex in self.v {
            vertex -= vector;
        }
    }
}
impl RTriangle3 {
    pub fn new() -> Self {
        RTriangle3{
            v: [RVector3::new(0, 0, 0.0); 3],
            color: Color{ rgb565: 0x000 }
        }
    }
}

#[derive(Clone, Copy)]
pub struct RLine3(pub [RVector3; 2]);

#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32
}
// TODO: default angle is pi/2 rotated on the x-axis. work that out please
impl Default for Quaternion {
    fn default() -> Self {
        Quaternion {
            w: 0.5713734,
            x: 0.77361715,
            y: 0.2203645,
            z: -0.16275544
        }
    }
}
impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Quaternion { w, x, y, z }
    }

    pub fn from_angles(x: f32, y: f32, z: f32) -> Self {
        let (cx, sx) = (cos(x/2.0), sin(x/2.0));
        let (cy, sy) = (cos(y/2.0), sin(y/2.0));
        let (cz, sz) = (cos(z/2.0), sin(z/2.0));

        Quaternion::new(
            cx*cy*cz + sx*sy*sz, 
            sx*cy*cz + cx*sy*sz, 
            cx*sy*cz + sx*cy*sz, 
            cx*cy*sz + sx*sy*cz
        )
    }

    pub fn get_rotation_matrix(&self) -> Matrix4 {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;
        Matrix4 ([
            [1.0 - 2.0*y2 - 2.0*z2, 2.0*x*y - 2.0*w*z    , 2.0*x*z + 2.0*w*y    , 0.0],
            [2.0*x*y + 2.0*w*z    , 1.0 - 2.0*x2 - 2.0*z2, 2.0*y*z - 2.0*w*x    , 0.0],
            [2.0*x*z - 2.0*w*y    , 2.0*y*z + 2.0*w*x    , 1.0 - 2.0*x2 - 2.0*y2, 0.0],
            [0.0                  , 0.0                  , 0.0                  , 1.0]
        ])
    }
}
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, o: Quaternion) -> Quaternion {
        let s = self;
        Quaternion {
            w: s.w * o.w - s.x * o.x - s.y * o.y - s.z * o.z,
            x: s.w * o.x + s.x * o.w + s.y * o.z - s.z * o.y,
            y: s.w * o.y - s.x * o.z + s.y * o.w + s.z * o.x,
            z: s.w * o.z + s.x * o.y - s.y * o.x + s.z * o.w
        }
    }
}
