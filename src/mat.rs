use crate::eadk::*;
use core::ops::{ AddAssign, SubAssign, MulAssign, Index, IndexMut };
#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(target_os = "none")]
use alloc::vec;

pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub indices: Vec<Vector3>,
    pub transformed_indices: Vec<RVector2>
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            tris: vec![],
            indices: vec![],
            transformed_indices: vec![]
        }
    }

    pub fn transform(&mut self) {
        self.transformed_indices.clear();
        for vertex in &self.indices {
            self.transformed_indices.push(RVector2::from_vector3(vertex));
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RVector2 {
    pub x: isize,
    pub y: isize
}
impl RVector2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn from_vector3 (vector3: &Vector3) -> Self {
        Self {
           x: vector3.x as isize,
           y: vector3.y as isize 
        }
    }
}
impl AddAssign for RVector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign for RVector2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
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
pub struct Triangle(pub [usize; 3]);


#[derive(Debug, Clone, Copy)]
pub struct Matrix4(pub [[f32; 4]; 4]);

#[derive(Debug, Clone, Copy)]
pub struct Matrix3(pub [[f32; 3]; 3]);
