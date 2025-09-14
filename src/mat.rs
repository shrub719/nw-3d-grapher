use crate::eadk::*;
use core::ops::{ AddAssign, SubAssign };

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: isize,
    pub y: isize
}
impl Vector2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Clone, Copy)]
pub struct RTriangle {
    pub vertices: [Vector2; 3],
    pub color: Color
}
impl AddAssign<Vector2> for RTriangle {
    fn add_assign(&mut self, point: Vector2) {
        for vertex in &mut self.vertices {
            *vertex += point;
        }
    }
}
impl SubAssign<Vector2> for RTriangle {
    fn sub_assign(&mut self, point: Vector2) {
        for vertex in &mut self.vertices {
            *vertex -= point;
        }
    }
}