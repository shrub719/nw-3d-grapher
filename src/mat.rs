pub type Matrix3 = [[f32; 3]; 3];
pub type Matrix4 = [[f32; 4]; 4];

#[derive(Clone, Copy, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}
impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy)]
pub struct Point4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}
impl<T> Point4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

