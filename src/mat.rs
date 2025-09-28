use core::ops::{ AddAssign, SubAssign, Mul, MulAssign, Index, IndexMut };

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
impl AddAssign for RVector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl SubAssign for RVector3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

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
impl Index<usize> for Vector3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range for Vector3"),
        }
    }
}
impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range for Vector3"),
        }
    }
}
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
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        for i in 0..4 {
            let mut sum: f32 = 0.0;
            for j in 0..4 {
                sum += matrix.0[i][j] * self_4[j];
            }
            if i != 3 {
                result[i] = sum;
            }
        }
        result
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
        let mut result = RTriangle3 ( [RVector3::new(0, 0, 0.0); 3] );
        let mut index: usize = 0;
        for vertex in self.0 {
            result.0[index] = RVector3::from_vector3(vertex * matrix);
        }
        result
    }
}

#[derive(Clone, Copy)]
pub struct RTriangle3(pub [RVector3; 3]);
