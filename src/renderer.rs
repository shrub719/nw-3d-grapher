use crate::{ mat::*, eadk::* };
use crate::{ config::* };

pub type Triangle2<T> = [Point2<T>; 3];
pub type Triangle3<T> = [Point3<T>; 3];

pub struct FrameBuffer {
    tile_row: u8,
    tile_column: u8,
    buffer: [[Color; FB_WIDTH]; FB_HEIGHT]
}

fn random_u16() -> u16 {
    return random() as u16;
}

fn random_point() -> Point2<u16> {
    return Point2 { x: random_u16(), y: random_u16() };
}

pub fn draw_screen() {
    let mut tris: [Triangle2::<u16>; 4] = [
        [
            Point2 { x: 0, y: 0 }; 3
        ]; 4
    ];
    for i in 0..4 {
        let v1 = random_point();
        let v2 = random_point();
        let v3 = random_point();
        tris[i] = [v1, v2, v3];
    }

    for tri in tris {
        draw_triangle(tri);
    }
}

pub fn draw_triangle(tri: Triangle2<u16>) {
    let [v1, v2, v3] = tri;
    let x = v1.x;
    let y = v1.y;
    let width = v2.x - v1.x;
    let height = v2.y - v1.y;

    let r = Rect{ x, y, width, height };
    let c = Color{ rgb565: 0xF00 };
    display::push_rect_uniform(r, c);
}