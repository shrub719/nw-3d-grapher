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
        while !self.input.keyboard_state.key_down(eadk::input::Key::Back) {
            let color = eadk::Color::from_rgb(eadk::random() as u16, eadk::random() as u16, eadk::random() as u16);

            self.input.update();
            if self.input.update_domain {
                // regen mesh
            }
            if self.input.update_rotation {
                self.mesh.rotate(self.input.rotation_direction);
            }

            self.mesh.transform();
            self.renderer.draw_screen(&self.mesh, color);

            while !(self.input.keyboard_state.key_down(eadk::input::Key::Ok) || self.input.keyboard_state.key_down(eadk::input::Key::Back)) {
                self.input.update();
                eadk::timing::msleep(50);
            }
        }
    }
}