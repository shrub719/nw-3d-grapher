use crate::{
    renderer::*,
    mesh::Mesh,
    mat::{ Vector3, Triangle3 },
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
        self.mesh.tris = [
            // square1
            [Triangle3 ([
                Vector3::new(-3.0, -3.0, 10.0), Vector3::new(-3.0, 3.0, 10.0), Vector3::new(3.0, 3.0, 10.0)
            ]); test::TEST_N], 
            [Triangle3 ([
                Vector3::new(-3.0, -3.0, 10.0), Vector3::new(3.0, -3.0, 10.0), Vector3::new(3.0, 3.0, 10.0)
            ]); test::TEST_N],

            // square2
            [Triangle3 ([
                Vector3::new(-3.0, -3.0, -10.0), Vector3::new(-3.0, 3.0, -10.0), Vector3::new(3.0, 3.0, -10.0)
            ]); test::TEST_N], 
            [Triangle3 ([
                Vector3::new(-3.0, -3.0, -10.0), Vector3::new(3.0, -3.0, -10.0), Vector3::new(3.0, 3.0, -10.0)
            ]); test::TEST_N],

            // side1
            [Triangle3 ([
                Vector3::new(-3.0, -3.0, 10.0), Vector3::new(-3.0, 3.0, 10.0), Vector3::new(-3.0, -3.0, -10.0)
            ]); test::TEST_N], 
            [Triangle3 ([
                Vector3::new(-3.0, 3.0, 10.0), Vector3::new(-3.0, -3.0, -10.0), Vector3::new(-3.0, 3.0, -10.0)
            ]); test::TEST_N],

            // side2
            [Triangle3 ([
                Vector3::new(-3.0, -3.0, 10.0), Vector3::new(3.0, -3.0, 10.0), Vector3::new(-3.0, -3.0, -10.0)
            ]); test::TEST_N], 
            [Triangle3 ([
                Vector3::new(3.0, -3.0, 10.0), Vector3::new(-3.0, -3.0, -10.0), Vector3::new(3.0, -3.0, -10.0)
            ]); test::TEST_N],
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