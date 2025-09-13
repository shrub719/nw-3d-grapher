use crate::{ mat::*, eadk::* };
use crate::{ config::* };

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

    for row in 0..FB_TILE {
        for column in 0.. FB_TILE {
            let mut frame_buffer = FrameBuffer::new(row, column);
            for tri in tris {
                draw_triangle(tri, &mut frame_buffer);
            }
            frame_buffer.push();
        }
    }
}

pub fn draw_triangle(tri: Triangle2<u16>, frame_buffer: &mut FrameBuffer) {
    // let [mut v0, mut v1, mut v2] = tri;
    
    // use core::mem::swap;
    // if v0.y > v1.y { swap(&mut v0, &mut v1) }
    // if v0.y > v2.y { swap(&mut v0, &mut v2) }
    // if v1.y > v2.y { swap(&mut v1, &mut v2) }
    for i in 0..FB_WIDTH*FB_HEIGHT {
        let b = ((i % FB_WIDTH) as f32 / FB_WIDTH as f32 * 255.0) as u16;
        let g = ((i / FB_WIDTH) as f32 / FB_HEIGHT as f32 * 255.0) as u16;
        frame_buffer.buffer[i] = Color::from_rgb(0, g, b);
    }
}