use crate::{
    renderer::*,
    mesh::Mesh,
    mat::{ Vector3, Triangle3 },
    input::*,
    timer::*,
    config::*,
    eadk::random
};
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::format;
use crate::eadk::info;

fn random_coord() -> f32 {
    (random() as u16 as f32) / (u16::MAX as f32) * 20.0 - 10.0
}

fn random_point() -> Vector3 {
    Vector3::new(random_coord(), random_coord(), random_coord())
}

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
        for i in 0..test::TEST_N {
            self.mesh.tris.push(
                Triangle3([
                    random_point(), random_point(), random_point()
                ])
            );
        }

        // main loop - runs every frame
        while !self.input.upd.quit {
            if self.input.upd.domain {
                self.mesh.update_domain();
            }
            if self.input.upd.rotation {
                self.mesh.update_rotation(self.input.rotation_direction, self.timer.delta_time);
            }

            if self.input.upd.redraw {
                info(&format!("tris: {} // fps: {:.1}", self.mesh.tris.len(), self.timer.get_fps()));
                self.mesh.transform();
                self.renderer.draw_screen(&self.mesh);
            }

            self.input.update();
            self.timer.update();
        }
    }
}