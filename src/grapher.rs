use crate::{
    renderer::*,
    mesh::*,
    mat::*,
    input::*,
    eadk
};

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