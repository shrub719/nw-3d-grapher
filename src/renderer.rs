use crate::{ eadk::*, config::*, mat::{ Vector3, Triangle3, Mesh3 } };
use core::ops::{ AddAssign, SubAssign };
#[cfg(target_os = "none")]
use alloc::format;
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

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

pub struct RTriangle2 {
    pub vertices: [RVector2; 3],
    pub color: Color
}
impl RTriangle2 {
    pub fn from_triangle3 (triangle3: &Triangle3, indices: &Vec<Vector3> ) -> Self {
        let mut vertices = [RVector2::new(0, 0); 3];
        for i in 0..3 {
            vertices[i] = RVector2::from_vector3(&indices[triangle3.0[i]]);
        }

        Self {
            vertices,
            color: Color::from_rgb(0, 255, 255)  // TODO: calculate color based on z
        }
    }
}
impl AddAssign<RVector2> for RTriangle2 {
    fn add_assign(&mut self, point: RVector2) {
        for vertex in &mut self.vertices {
            *vertex += point;
        }
    }
}
impl SubAssign<RVector2> for RTriangle2 {
    fn sub_assign(&mut self, point: RVector2) {
        for vertex in &mut self.vertices {
            *vertex -= point;
        }
    }
}

// frame buffer split into several tiles each frame to accommodate for small memory
pub struct FrameBuffer {
    row: usize,
    column: usize,
    offset_vector: RVector2,
    buffer: [Color; FB_WIDTH * FB_HEIGHT]
}
impl FrameBuffer {
    pub fn new() -> Self {
        Self { 
            row: 0,
            column: 0,
            offset_vector: RVector2::new(0, 0),
            buffer: [Color{ rgb565: 0x000 }; FB_WIDTH * FB_HEIGHT]
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [Color{ rgb565: 0x000 }; FB_WIDTH * FB_HEIGHT];
    }

    pub fn set_offset(&mut self, row: usize, column: usize) {
        self.row = row;
        self.column = column;
        self.offset_vector = RVector2::new((self.column * FB_WIDTH) as isize, (self.row * FB_HEIGHT) as isize);
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

fn random_u16() -> u16 {
    return random() as u16;
}

fn random_coordinate() -> u16 {
    return (random() % 0xFF) as u16;
}

fn random_point() -> RVector2 {
    return RVector2 { x: random_coordinate() as isize, y: random_coordinate() as isize };
}

pub fn draw_screen() {
    // let mut indices = [
    //     Vector3::new(random_coordinate() as f32, 0.0, 50.0),
    //     Vector3::new(0.4, 100.0, 5.09),
    //     Vector3::new(100.0, 74.0, -1.0)
    // ];
    // let mut tris = [Triangle3 ([
    //     0, 1, 2
    // ]); TEST_N];

    let mesh = Mesh3 {
        indices: vec![
            Vector3::new(random_coordinate() as f32, 0.0, 50.0),
            Vector3::new(0.4, 100.0, 5.09),
            Vector3::new(100.0, 74.0, -1.0)
        ],
        tris: vec![Triangle3 ([0, 1, 2]); TEST_N]
    };

    // debug_info(&format!("{:?}", tris), 1000);

    // loops through tiles on the screen, rendering each tile separately
    let mut frame_buffer = FrameBuffer::new();
    for row in 0..FB_TILE {
        for column in 0.. FB_TILE {
            frame_buffer.clear();
            frame_buffer.set_offset(row, column);
            for tri in &mesh.tris {
                fill_triangle(RTriangle2::from_triangle3(&tri, &mesh.indices), &mut frame_buffer);
            }
            frame_buffer.push();
        }
    }
    display::wait_for_vblank();
}

fn fill_triangle(mut tri: RTriangle2, frame_buffer: &mut FrameBuffer) {
    tri -= frame_buffer.offset_vector;

    let [mut v0, mut v1, mut v2] = tri.vertices;
    let color = tri.color;

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