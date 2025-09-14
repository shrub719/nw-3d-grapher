use crate::{ eadk::*, config::*, mat::{ Vector2, Triangle2 } };
use alloc::format;

// frame buffer split into several tiles each frame to accommodate for small memory
pub struct FrameBuffer {
    row: usize,
    column: usize,
    offset_vector: Vector2,
    buffer: [Color; FB_WIDTH * FB_HEIGHT]
}
impl FrameBuffer {
    pub fn new() -> Self {
        Self { 
            row: 0,
            column: 0,
            offset_vector: Vector2::new(0, 0),
            buffer: [Color{ rgb565: 0x000 }; FB_WIDTH * FB_HEIGHT]
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [Color{ rgb565: 0x000 }; FB_WIDTH * FB_HEIGHT];
    }

    pub fn set_offset(&mut self, row: usize, column: usize) {
        self.row = row;
        self.column = column;
        self.offset_vector = Vector2::new((self.column * FB_WIDTH) as isize, (self.row * FB_HEIGHT) as isize);
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

fn random_point() -> Vector2 {
    return Vector2 { x: random_coordinate() as isize, y: random_coordinate() as isize };
}

pub fn draw_screen() {
    let mut tris: [Triangle2; TEST_N] = [Triangle2 {
            vertices: [random_point(), random_point(), random_point()],
            color: Color::from_rgb(0, 255, 255)
        }; TEST_N
    ];

    // debug_info(&format!("{:?}", tris), 1000);

    // loops through tiles on the screen, rendering each tile separately
    let mut frame_buffer = FrameBuffer::new();
    for row in 0..FB_TILE {
        for column in 0.. FB_TILE {
            frame_buffer.clear();
            frame_buffer.set_offset(row, column);
            for tri in tris {
                fill_triangle(tri, &mut frame_buffer);
            }
            frame_buffer.push();
        }
    }
    display::wait_for_vblank();
}

fn fill_triangle(mut tri: Triangle2, frame_buffer: &mut FrameBuffer) {
    tri -= frame_buffer.offset_vector;

    let [mut t0, mut t1, mut t2] = tri.vertices;
    let color = tri.color;

    use core::mem::swap;
    if t0.y > t1.y { swap(&mut t0, &mut t1) }
    if t0.y > t2.y { swap(&mut t0, &mut t2) }
    if t1.y > t2.y { swap(&mut t1, &mut t2) }

    let triangle_height = t2.y - t0.y;
    let triangle_heightf = triangle_height as f32;

    'height_iter: for i in 0..triangle_height {
        let second_half = i > (t1.y - t0.y) || (t1.y == t0.y);
        let segment_heightf = if second_half {
            (t2.y - t1.y) as f32
        } else {
            (t1.y - t0.y) as f32
        };

        let alpha = i as f32 / triangle_heightf;
        let beta = if second_half {
            (i as f32 - (t1.y - t0.y) as f32) / segment_heightf
        } else {
            i as f32 / segment_heightf
        };

        let mut a = t0.x as f32 + ((t2.x - t0.x) as f32 * alpha);
        let mut b = if second_half {
            t1.x as f32 + ((t2.x - t1.x) as f32 * beta)
        } else {
            t0.x as f32 + ((t1.x - t0.x) as f32 * beta)
        };

        if a > b {
            swap(&mut a, &mut b);
        }

        let y = t0.y + i;
        if y < 0 {
            continue 'height_iter;
        }
        if y >= FB_HEIGHT as isize {
            break 'height_iter;
        }

        if (b as usize) < 1 {
            // prevent line bug
            continue;
        }

        for j in (a as usize)..=(b as usize) {
            if j >= FB_WIDTH as usize {
                continue 'height_iter;
            }
            frame_buffer.set_pixel(j, y as usize, color);
        }
    }
}