use crate::{
    renderer::*,
    mesh::Mesh,
    input::*,
    hud::*,
    timer::*,
    eadk::*,
    config::{ graphics::*, palette::* }
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
        display::push_rect_uniform(
            Rect {
                x: 0,
                y: MARGIN_TOP,
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT - MARGIN_TOP
            },
            WHITE
        );

        display::push_rect_uniform(
            Rect {
                x: 0,
                y: 0,
                width: SCREEN_WIDTH - 20,
                height: MARGIN_TOP
            },
            ORANGE
        );
        display::draw_string(
            "3D GRAPHER",
            Point { x: 122, y: 3 },
            false,
            WHITE,
            ORANGE
        );

        // main loop - runs every frame
        while !self.input.upd.quit {
            if self.input.upd.domain {
                self.mesh.update_domain(self.input.domain_trans_direction, self.input.domain_scale_direction);
                self.mesh.generate_mesh();
                self.input.domain_cooldown = 0.0;
            } else {
                self.input.domain_cooldown += self.timer.delta_time;
            }
            if self.input.upd.load_obj {
                self.mesh.load_mesh_from_file();
            }
            
            if self.input.upd.rotation {
                self.mesh.update_rotation(self.input.rotation_direction, self.timer.delta_time);
            }
            if self.input.upd.scale {
                self.mesh.update_scale(self.input.scale_change, self.timer.delta_time);
            }

            if self.input.upd.hud {
                draw_hud(self.input.mode, self.input.upd.mode, self.input.upd.help_on, self.mesh.scale, self.mesh.tris.len());
            }

            if self.input.upd.redraw {
                info(&format!("fps: {:.1}   ", self.timer.get_fps()));
                self.mesh.transform();
                self.renderer.draw_screen(&self.mesh, self.input.help);
            }

            self.input.update();
            self.timer.update();
        }
    }
}