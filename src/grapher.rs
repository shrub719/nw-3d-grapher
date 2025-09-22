use crate::{
    renderer::*,
    config::*,
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
            let color = eadk::Color::from_rgb(eadk::random() as u16, eadk::random() as u16, eadk::random() as u16);

            self.input.update();
            if self.input.upd.domain {
                // regen mesh
            }
            if self.input.upd.rotation {
                self.mesh.update_rotation(self.input.rotation_direction);
            }

            self.mesh.transform();
            self.renderer.draw_screen(&self.mesh, color);

            while !self.input.upd.cont {
                self.input.update();
                eadk::timing::msleep(50);
            }
        }
    }
}