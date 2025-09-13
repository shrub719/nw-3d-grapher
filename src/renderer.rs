use crate::{ mat::*, eadk::* };
use crate::{ config::* };
use alloc::format;

pub type Triangle2<T> = [Point2<T>; 3];
pub type Triangle3<T> = [Point3<T>; 3];

pub struct FrameBuffer {
    tile_row: usize,
    tile_column: usize,
    buffer: [Color; FB_WIDTH * FB_HEIGHT]
}
impl FrameBuffer {
    pub fn new(tile_row: usize, tile_column: usize) -> Self {
        Self { 
            tile_row,
            tile_column,
            buffer: [Color{ rgb565: 0x000 }; FB_WIDTH * FB_HEIGHT]
        }
    }

    pub fn push(&self) {
        display::push_rect(
            Rect{ 
                x: (self.tile_column * FB_WIDTH) as u16,
                y: (self.tile_row * FB_HEIGHT) as u16,
                width: FB_WIDTH as u16,
                height: FB_HEIGHT as u16
            },
            &self.buffer
        );
    }

    pub fn set_pixel(&mut self, mut x: usize, mut y: usize, color: Color) {
        x -= self.tile_column * FB_WIDTH;
        y -= self.tile_row * FB_HEIGHT;
        let index = y * FB_WIDTH + x;
        self.buffer[index] = color;
    }
}

fn random_u16() -> u16 {
    return random() as u16;
}

fn random_point() -> Point2<u16> {
    return Point2::new(random_u16(), random_u16());
}

pub fn draw_screen() {
    // let mut tris: [Triangle2::<u16>; 4] = [
    //     [
    //         Point2 { x: 0, y: 0 }; 3
    //     ]; 4
    // ];
    // for i in 0..4 {
    //     let v1 = random_point();
    //     let v2 = random_point();
    //     let v3 = random_point();
    //     tris[i] = [v1, v2, v3];
    // }
    let mut tris: [Triangle2::<u16>; 4] = [
        [
            Point2::new(10, 10),
            Point2::new(50, 0),
            Point2::new(30, 30)
        ]; 4
    ];

    // for row in 0..FB_TILE {
    //     for column in 0.. FB_TILE {
    //         let mut frame_buffer = FrameBuffer::new(row, column);
    //         for tri in tris {
    //             fill_triangle(tri, Color::from_rgb(0, 255, 255), &mut frame_buffer);
    //         }
    //         frame_buffer.push();
    //     }
    // }
    let mut frame_buffer = FrameBuffer::new(0, 0);
    fill_triangle(tris[0], Color::from_rgb(0, 255, 255), &mut frame_buffer);
    frame_buffer.push();
}

fn fill_triangle(tri: Triangle2<u16>, color: Color, frame_buffer: &mut FrameBuffer) {
    // for i in 0..FB_WIDTH*FB_HEIGHT {
    //     let b = ((i % FB_WIDTH) as f32 / FB_WIDTH as f32 * 255.0) as u16;
    //     let g = ((i / FB_WIDTH) as f32 / FB_HEIGHT as f32 * 255.0) as u16;
    //     frame_buffer.buffer[i] = Color::from_rgb(0, g, b);
    // }
    let [mut v0, mut v1, mut v2] = tri;
    
    use core::mem::swap;
    if v0.y > v1.y { swap(&mut v0, &mut v1) }
    if v0.y > v2.y { swap(&mut v0, &mut v2) }
    if v1.y > v2.y { swap(&mut v1, &mut v2) }

    debug_info(&format!(r#"
{} {}
{} {}
{} {}
        "#, v0.x, v0.y, v1.x, v1.y, v2.x, v2.y), 1000);

    if v1.y == v2.y {
        fill_bottom_flat_tri([v0, v1, v2], color, frame_buffer);
    }
    else if v0.y == v1.y {
        fill_top_flat_tri([v0, v1, v2], color, frame_buffer);
    }
    else {
        let mut v3 = Point2::new(
            (v0.x as f32 + (
                (
                    (v1.y as f32 - v0.y as f32) / (v2.y as f32 - v0.y as f32)
                ) * (v2.x as f32 - v0.x as f32)
            )) as u16, 
            v1.y
        );
        debug_info(&format!("{} {}", v3.x, v3.y), 1000);
        fill_bottom_flat_tri([v0, v1, v3], color, frame_buffer);
        fill_top_flat_tri([v1, v3, v2], color, frame_buffer);
    }
}

fn fill_bottom_flat_tri(tri: Triangle2<u16>, color: Color, frame_buffer: &mut FrameBuffer) {
    let [v0, v1, v2] = tri;

    let inv_m1 = (v1.x as f32 - v0.x as f32) / (v1.y as f32 - v0.y as f32);
    let inv_m2 = (v2.x as f32 - v0.x as f32) / (v2.y as f32 - v0.y as f32);

    let mut cur_x1 = v0.x as f32;
    let mut cur_x2 = v0.x as f32;
    
    for cur_y in v0.y..=v1.y {
        draw_line(cur_x1 as u16, cur_y, cur_x2 as u16, cur_y, color, frame_buffer);
        cur_x1 += inv_m1;
        cur_x2 += inv_m2;
    }
}

fn fill_top_flat_tri(tri: Triangle2<u16>, color: Color, frame_buffer: &mut FrameBuffer) {
    let [v0, v1, v2] = tri;

    let inv_m1 = (v2.x as f32 - v0.x as f32) / (v2.y as f32 - v0.y as f32);
    let inv_m2 = (v2.x as f32 - v1.x as f32) / (v2.y as f32 - v1.y as f32);

    let mut cur_x1 = v2.x as f32;
    let mut cur_x2 = v2.x as f32;
    
    for cur_y in (v0.y..=v2.y).rev() {
        draw_line(cur_x1 as u16, cur_y, cur_x2 as u16, cur_y, color, frame_buffer);
        cur_x1 -= inv_m1;
        cur_x2 -= inv_m2;
    }
}

fn draw_line(x1: u16, y1: u16, x2: u16, y2: u16, color: Color, frame_buffer: &mut FrameBuffer) {
    for x in x1..=x2 {
        frame_buffer.set_pixel(x as usize, y1 as usize, color);
    }
}
