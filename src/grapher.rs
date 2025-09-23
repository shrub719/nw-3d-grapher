use crate::{
    renderer::*,
    mesh::{ *, Triangle },
    mat::Vector3,
    input::*,
    config::*
};
#[cfg(target_os = "none")]
use alloc::vec;

pub struct Grapher {
    renderer: Renderer,
    mesh: Mesh,
    input: InputHandler
}
impl Grapher {
    pub fn new() -> Self {
        Grapher {
            renderer: Renderer::new(),
            mesh: Mesh::new(),
            input: InputHandler::new(),
        }
    }

    pub fn main_loop(&mut self) {
        self.mesh.indices = vec![
            Vector3::new(-3.0, -3.0, 10.0),
            Vector3::new(-3.0, 3.0, 10.0),
            Vector3::new(3.0, -3.0, 10.0),
            Vector3::new(3.0, 3.0, 10.0),
            Vector3::new(-3.0, -3.0, -10.0),
            Vector3::new(-3.0, 3.0, -10.0),
            Vector3::new(3.0, -3.0, -10.0),
            Vector3::new(3.0, 3.0, -10.0),
        ];
        self.mesh.tris = [
            // square1
            [Triangle ([0, 1, 3]); test::TEST_N], 
            [Triangle ([0, 2, 3]); test::TEST_N],

            // square2
            [Triangle ([4, 5, 7]); test::TEST_N], 
            [Triangle ([4, 6, 7]); test::TEST_N],

            // side1
            [Triangle ([0, 1, 4]); test::TEST_N], 
            [Triangle ([1, 4, 5]); test::TEST_N],

            // side2
            [Triangle ([0, 2, 4]); test::TEST_N], 
            [Triangle ([2, 4, 6]); test::TEST_N],
        ].concat().to_vec();

        // main loop - runs every frame
        while !self.input.upd.quit {
            if self.input.upd.domain {
                self.mesh.update_domain();
            }
            if self.input.upd.rotation {
                self.mesh.update_rotation(self.input.rotation_direction);
            }

            self.mesh.transform();
            self.renderer.draw_screen(&self.mesh);

            self.input.update();
        }
    }
}