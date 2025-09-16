use crate::{ eadk::*, config::*, mat::{ RVector3, Vector3 }, mesh::{ Mesh, Triangle, Line } };
use core::ops::{ AddAssign, SubAssign };
#[cfg(target_os = "none")]
use alloc::format;

// frame buffer split into several tiles each frame to accommodate for small memory
pub struct FrameBuffer {
    row: usize,
    column: usize,
    offset_vector: RVector3,
    buffer: [Color; FB_WIDTH * FB_HEIGHT]
}
impl FrameBuffer {
    pub fn new() -> Self {
        Self { 
            row: 0,
            column: 0,
            offset_vector: RVector3::new(0, 0, 0.0),
            buffer: [Color{ rgb565: 0x000 }; FB_WIDTH * FB_HEIGHT]
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [Color{ rgb565: 0x000 }; FB_WIDTH * FB_HEIGHT];
    }

    pub fn set_offset(&mut self, row: usize, column: usize) {
        self.row = row;
        self.column = column;
        self.offset_vector = RVector3::new((self.column * FB_WIDTH) as isize, (self.row * FB_HEIGHT) as isize, 0.0);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[y * FB_WIDTH + x] = color;
    }

    pub fn push(&self) {
        display::push_rect(
            Rect{ 
                x: (self.column * FB_WIDTH) as u16,
                y: (self.row * FB_HEIGHT) as u16,
                width: (FB_WIDTH) as u16,
                height: (FB_HEIGHT) as u16
            },
            &self.buffer
        );
    }
}

pub fn draw_screen(mesh: &Mesh, color: Color) {
    // let mut indices = [
    //     Vector3::new(random_coordinate() as f32, 0.0, 50.0),
    //     Vector3::new(0.4, 100.0, 5.09),
    //     Vector3::new(100.0, 74.0, -1.0)
    // ];
    // let mut tris = [Triangle ([
    //     0, 1, 2
    // ]); TEST_N];

    // debug_info(&format!("{:?}", tris), 1000);

    // loops through tiles on the screen, rendering each tile separately
    let mut frame_buffer = FrameBuffer::new();
    for column in (0..FB_TILE).rev() {
        for row in 0..FB_TILE {
            frame_buffer.clear();
            frame_buffer.set_offset(row, column);
            for tri in &mesh.tris {
                fill_triangle(
                    mesh.transformed_indices[tri.0[0]],
                    mesh.transformed_indices[tri.0[1]],
                    mesh.transformed_indices[tri.0[2]],
                    &mut frame_buffer,
                    color);
            }
            frame_buffer.push();
        }
    }
    display::wait_for_vblank();
}

fn fill_triangle(mut v0: RVector3, mut v1: RVector3, mut v2: RVector3, frame_buffer: &mut FrameBuffer, color: Color) {
    v0 -= frame_buffer.offset_vector;
    v1 -= frame_buffer.offset_vector;
    v2 -= frame_buffer.offset_vector;

    use core::mem::swap;
    if v0.y > v1.y { swap(&mut v0, &mut v1) }
    if v0.y > v2.y { swap(&mut v0, &mut v2) }
    if v1.y > v2.y { swap(&mut v1, &mut v2) }

    let triangle_height = v2.y - v0.y;
    let triangle_heightf = triangle_height as f32;

    'height_iter: for y_scan in 0..triangle_height {
        let is_second_half = y_scan > (v1.y - v0.y) || (v1.y == v0.y);
        let segment_heightf = if is_second_half {
            (v2.y - v1.y) as f32
        } else {
            (v1.y - v0.y) as f32
        };

        let height_progress = y_scan as f32 / triangle_heightf;
        let segment_progress = if is_second_half {
            (y_scan as f32 - (v1.y - v0.y) as f32) / segment_heightf
        } else {
            y_scan as f32 / segment_heightf
        };

        let mut x_left = v0.x as f32 + ((v2.x - v0.x) as f32 * height_progress);
        let mut x_right = if is_second_half {
            v1.x as f32 + ((v2.x - v1.x) as f32 * segment_progress)
        } else {
            v0.x as f32 + ((v1.x - v0.x) as f32 * segment_progress)
        };

        if x_left > x_right {
            swap(&mut x_left, &mut x_right);
        }

        let y = v0.y + y_scan;
        if y < 0 {
            continue 'height_iter;
        }
        if y >= FB_HEIGHT as isize {
            break 'height_iter;
        }

        if (x_right as usize) < 1 {
            continue 'height_iter;
        }

        for x_scan in (x_left as usize)..=(x_right as usize) {
            if x_scan >= FB_WIDTH as usize {
                continue 'height_iter;
            }
            frame_buffer.set_pixel(x_scan, y as usize, color);
        }
    }
}