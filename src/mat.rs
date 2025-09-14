use crate::eadk::*;
use core::ops::{ AddAssign, SubAssign, MulAssign, Index, IndexMut };
#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(target_os = "none")]
use alloc::vec;

pub struct Mesh3 {
    pub tris: Vec<Triangle3>,
    pub indices: Vec<Vector3>
}
impl Mesh3 {
    pub fn new() -> Self {
        Self {
            tris: vec![],
            indices: vec![]
        }
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
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range for Vector3"),
        }
    }
}
impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range for Vector3"),
        }
    }
}
impl MulAssign<Matrix3> for Vector3 {
    fn mul_assign(&mut self, matrix: Matrix3) {
        let self_copy = *self;
        for i in 0..3 {
            let mut sum: f32 = 0.0;
            for j in 0..3 {
                sum += matrix.0[i][j] * self_copy[j];
            }
            self[i] = sum;
        }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle3(pub [usize; 3]);


#[derive(Debug, Clone, Copy)]
pub struct Matrix4(pub [[f32; 4]; 4]);

#[derive(Debug, Clone, Copy)]
pub struct Matrix3(pub [[f32; 3]; 3]);
