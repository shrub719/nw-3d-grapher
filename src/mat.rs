pub type Matrix3 = [[f32; 3]; 3];
pub type Matrix4 = [[f32; 4]; 4];

pub struct Point2<T> {
    pub x: T,
    pub y: T,
}
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}
pub struct Point4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}
