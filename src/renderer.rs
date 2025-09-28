use crate::{ eadk::*, config::graphics::*, mat::RVector3, mesh::Mesh };

pub struct Renderer {
    buffer: [Color; FB_WIDTH * FB_HEIGHT],
    depth_buffer: [f16; FB_WIDTH * FB_HEIGHT],
}
impl Renderer {
    pub fn new() -> Self {
        Renderer {
            buffer: [Color{ rgb565: 0x000 }; FB_WIDTH * FB_HEIGHT],
            depth_buffer: [10000.0; FB_WIDTH * FB_HEIGHT]
        }
    }

    pub fn clear(&mut self) {
        for px in self.buffer.iter_mut() {
            *px = Color { rgb565: 0x000 };
        }
        for d in self.depth_buffer.iter_mut() {
            *d = 10000.0; // TODO: stop with this nonsense
        }
    }

    pub fn draw_screen(&mut self, mesh: &Mesh) {
        for column in 0..FB_TILE {
            for row in 0..FB_TILE {
                self.clear();
                let offset_vector = RVector3::new(
                    (MARGIN_SIDE + column * FB_WIDTH) as isize, 
                    (MARGIN_TOP + row * FB_HEIGHT) as isize, 
                    0.0
                );
                let mut value: u16 = 0;
                let inc = (255 / &mesh.tris.len()) as u16;
                for tri in &mesh.transformed_tris {
                    let color = Color::from_rgb(value, value, 255);
                    self.fill_triangle(
                        tri.0[0],
                        tri.0[1],
                        tri.0[2],
                        offset_vector,
                        color
                    );
                    value += inc;
                }
                display::push_rect(
                    Rect { 
                        x: offset_vector.x as u16,
                        y: offset_vector.y as u16,
                        width: (FB_WIDTH) as u16,
                        height: (FB_HEIGHT) as u16
                    },
                    &self.buffer
                );
            }
        }
        display::wait_for_vblank();
    }

    fn fill_triangle(&mut self, mut v0: RVector3, mut v1: RVector3, mut v2: RVector3, offset_vector: RVector3, color: Color) {
        v0 -= offset_vector;
        v1 -= offset_vector;
        v2 -= offset_vector;

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
            let mut z_left = v0.z + ((v2.z - v0.z) * height_progress);

            let mut x_right;
            let mut z_right;
            if is_second_half {
                x_right = v1.x as f32 + ((v2.x - v1.x) as f32 * segment_progress);
                z_right = v1.z + ((v2.z - v1.z) * segment_progress);
            } else {
                x_right = v0.x as f32 + ((v1.x - v0.x) as f32 * segment_progress);
                z_right = v0.z + ((v1.z - v0.z) * segment_progress);
            }

            if x_left > x_right {
                swap(&mut x_left, &mut x_right);
                swap(&mut z_left, &mut z_right);
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

            let scan_width = x_right - x_left;
            for x_scan in (x_left as usize)..=(x_right as usize) {
                if x_scan >= FB_WIDTH as usize {
                    continue 'height_iter;
                }

                let scan_progress = if scan_width != 0.0 {
                    (x_scan as f32 - x_left) / scan_width
                } else {
                    0.0
                };
                let z = (z_left + (z_right - z_left) * scan_progress) as f16;

                let index = y as usize * FB_WIDTH + x_scan;
                let curr_depth = self.depth_buffer[index];
                if z < curr_depth {
                    self.buffer[index] = color;
                    self.depth_buffer[index] = z;
                }
            }
        }
    }
}
