use crate::{ eadk::*, config::* };
use alloc::format;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16
}
impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    vertices: [Point; 3],
    color: Color
}

// frame buffer split into several tiles each frame to accommodate for small memory
pub struct FrameBuffer {
    tile_row: u16,
    tile_column: u16,
    buffer: [Color; (FB_WIDTH * FB_HEIGHT) as usize]
}
impl FrameBuffer {
    pub fn new(tile_row: u16, tile_column: u16) -> Self {
        Self { 
            tile_row,
            tile_column,
            buffer: [Color{ rgb565: 0x000 }; (FB_WIDTH * FB_HEIGHT) as usize]
        }
    }

    pub fn push(&self) {
        display::push_rect(
            Rect{ 
                x: self.tile_column * FB_WIDTH,
                y: self.tile_row * FB_HEIGHT,
                width: FB_WIDTH,
                height: FB_HEIGHT
            },
            &self.buffer
        );
    }

    pub fn set_pixel(&mut self, mut x: u16, mut y: u16, color: Color) {
        x -= self.tile_column * FB_WIDTH;
        y -= self.tile_row * FB_HEIGHT;
        let index = y * FB_WIDTH + x;
        self.buffer[index as usize] = color;
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        let x0 = self.tile_column * FB_WIDTH;
        let x1 = x0 + FB_WIDTH;
        let y0 = self.tile_row * FB_HEIGHT;
        let y1 = y0 + FB_HEIGHT;

        (x0 <= x && x < x1) && (y0 <= y && y < y1)
    }
}

fn random_u16() -> u16 {
    return random() as u16;
}

fn random_coordinate() -> u16 {
    return (random() % 0xFF) as u16;
}

fn random_point() -> Point {
    return Point { x: random_coordinate(), y: random_coordinate() };
}


pub fn draw_screen() {
    let mut tris: [Triangle; TEST_N] = [Triangle {
            vertices: [random_point(), random_point(), random_point()],
            color: Color::from_rgb(0, 255, 255)
        }; TEST_N];

    // debug_info(&format!("{:?}", tris), 1000);

    // loops through tiles on the screen, rendering each tile separately
    for row in 0..FB_TILE {
        for column in 0.. FB_TILE {
            let mut frame_buffer = FrameBuffer::new(row, column);
            for tri in tris {
                fill_triangle(tri, &mut frame_buffer);
            }
            frame_buffer.push();
        }
    }
    display::wait_for_vblank();
}

fn fill_triangle(tri: Triangle, frame_buffer: &mut FrameBuffer) {
    // TODO: triangle culling/splitting on frame buffer edges
    // TODO: fix some triangles appearing only as their vertices
    let [mut v0, mut v1, mut v2] = tri.vertices;
    let color = tri.color;
    
    // sort in ascending order of y value
    use core::mem::swap;
    if v0.y > v1.y { swap(&mut v0, &mut v1) }
    if v0.y > v2.y { swap(&mut v0, &mut v2) }
    if v1.y > v2.y { swap(&mut v1, &mut v2) }

    // debug_info(&format!(r#"
    // {} {}
    // {} {}
    // {} {}
    // "#, v0.x, v0.y, v1.x, v1.y, v2.x, v2.y), 1000);

    // account for bottom/top edge cases
    if v1.y == v2.y {
        fill_bottom_flat_tri(v0, v1, v2, color, frame_buffer);
    }
    else if v0.y == v1.y {
        fill_top_flat_tri(v0, v1, v2, color, frame_buffer);
    }
    // split triangle into flat bottom/top
    else {
        let mut v3 = Point::new(
            interpolate_coord(v0.x, v2.x, v0.y, v2.y, v1.y), 
            v1.y
        );
        // debug_info(&format!("{} {}", v3.x, v3.y), 1000);
        fill_bottom_flat_tri(v0, v1, v3, color, frame_buffer);
        fill_top_flat_tri(v1, v3, v2, color, frame_buffer);
    }
}

fn fill_bottom_flat_tri(v0: Point, v1: Point, v2: Point, color: Color, frame_buffer: &mut FrameBuffer) {
    let inv_m1 = (v1.x as f32 - v0.x as f32) / (v1.y as f32 - v0.y as f32);
    let inv_m2 = (v2.x as f32 - v0.x as f32) / (v2.y as f32 - v0.y as f32);

    let mut cur_x1 = v0.x as f32;
    let mut cur_x2 = v0.x as f32;
    
    for cur_y in v0.y..=v1.y {
        draw_scanline(cur_x1 as u16, cur_x2 as u16, cur_y, color, frame_buffer);
        cur_x1 += inv_m1;
        cur_x2 += inv_m2;
    }
}

fn fill_top_flat_tri(v0: Point, v1: Point, v2: Point, color: Color, frame_buffer: &mut FrameBuffer) {
    let inv_m1 = (v2.x as f32 - v0.x as f32) / (v2.y as f32 - v0.y as f32);
    let inv_m2 = (v2.x as f32 - v1.x as f32) / (v2.y as f32 - v1.y as f32);

    let mut cur_x1 = v2.x as f32;
    let mut cur_x2 = v2.x as f32;
    
    for cur_y in (v0.y..=v2.y).rev() {
        draw_scanline(cur_x1 as u16, cur_x2 as u16, cur_y, color, frame_buffer);
        cur_x1 -= inv_m1;
        cur_x2 -= inv_m2;
    }
}

fn interpolate_coord(x0: u16, x1: u16, y0: u16, y1: u16, y_interpolate: u16) -> u16 {
        (x0 as f32 + (
            (
                (y_interpolate as f32 - y0 as f32) / (y1 as f32 - y0 as f32)
            ) * (x1 as f32 - x0 as f32)
        )) as u16
}

fn draw_scanline(x1: u16, x2: u16, y: u16, color: Color, frame_buffer: &mut FrameBuffer) {
    for x in x1..=x2 {
        if frame_buffer.contains(x, y) {
            frame_buffer.set_pixel(x, y, color);
        }
    }
}
