use crate::{ mat::*, eadk::* };

pub type Triangle2<T> = [Point2<T>; 3];
pub type Triangle3<T> = [Poin3<T>; 3];

pub fn fill_triangle(
    v1: Point2<u16>,
    v2: Point2<u16>,
    v3: Point2<u16>
) {
    let x = v1.x;
    let y = v1.y;
    let width = v2.x - v1.x;
    let height = v2.y - v1.y;

    let r = Rect{ x, y, width, height };
    let c = Color{ rgb565: 0xF00 };
    display::push_rect_uniform(r, c);
}