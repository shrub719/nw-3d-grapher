use crate::{
    renderer::*,
    mesh::Mesh,
    input::*,
    timer::*,
};
#[cfg(target_os = "none")]
use alloc::format;
use crate::eadk::info;

pub struct Grapher {
    renderer: Renderer,
    mesh: Mesh,
    input: InputHandler,
    timer: Timer
}
impl Grapher {
    pub fn new() -> Self {
        Grapher {
            renderer: Renderer::new(),
            mesh: Mesh::new(),
            input: InputHandler::new(),
            timer: Timer::new()
        }
    }

    pub fn main_loop(&mut self) {
        // main loop - runs every frame
        while !self.input.upd.quit {
            if self.input.upd.domain {
                self.mesh.update_domain();
                self.mesh.generate_mesh(self.input.n_change);
            }
            if self.input.upd.rotation {
                self.mesh.update_rotation(self.input.rotation_direction, self.timer.delta_time);
            }
            if self.input.upd.scale {
                self.mesh.update_scale(self.input.scale_change, self.timer.delta_time);
            }

            if self.input.upd.redraw {
                info(&format!("tris: {} // fps: {:.1}                   ", self.mesh.tris.len(), self.timer.get_fps()));
                self.mesh.transform();
                self.renderer.draw_screen(&self.mesh);
            }

            self.input.update();
            self.timer.update();
        }
    }
}