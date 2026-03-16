use crate::{
    grapher::{
        renderer::*,
        mesh::Mesh,
        input::*,
        hud::*,
        timer::*
    },
    eadk::*,
    constants::{ graphics::*, palette::* },
    expr::parser::Expr
};
#[cfg(target_os = "none")]
use alloc::format;

pub mod mat;
mod renderer;
pub mod mesh;
mod input;
mod hud;
mod timer;

pub struct Grapher {
    renderer: Renderer,
    pub mesh: Mesh,
    input: InputHandler,
    timer: Timer,
    pub expr: Expr
}
impl Grapher {
    pub fn new(expr: Expr) -> Self {
        Grapher {
            renderer: Renderer::new(),
            mesh: Mesh::new(),
            input: InputHandler::new(),
            timer: Timer::new(),
            expr
        }
    }

    pub fn setup_ui() {
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

        #[cfg(not(target_os = "none"))]
        {
            display::push_rect_uniform(
                Rect {
                    x: SCREEN_WIDTH - 20,
                    y: 0,
                    width: 20,
                    height: MARGIN_TOP
                },
                ORANGE
            );
            display::draw_string(
                "sim",
                Point { x: 295, y: 3 },
                false,
                WHITE,
                ORANGE
            );
        }

        #[cfg(debug_assertions)]
        display::draw_string(
            "(dev)",
            Point { x: 255, y: 3 },
            false,
            WHITE,
            ORANGE
        );
    }

    pub fn main_loop(&mut self) {
        Grapher::setup_ui();

        // main loop - runs every frame
        while !self.input.upd.quit {
            if self.input.upd.domain {
                self.mesh.update_domain(self.input.domain_trans_direction, self.input.domain_scale_direction);
                // temp: check for exp
                self.mesh.tris.clear();

                if self.expr.is_implicit {
                    self.generate_mesh_imp();
                } else {
                    self.generate_mesh_exp();
                }

                self.input.domain_cooldown = 0.0;
            } else {
                self.input.domain_cooldown += self.timer.delta_time;
            }
            
            if self.input.upd.rotation {
                self.mesh.update_rotation(self.input.rotation_direction, self.timer.delta_time);
            }
            if self.input.upd.scale {
                self.mesh.update_scale(self.input.scale_change, self.timer.delta_time);
            }

            if self.input.upd.hud {
                draw_hud(self.input.mode, self.input.upd.mode, self.input.upd.help_on, self.mesh.scale, self.mesh.domain);
            }

            if self.input.upd.redraw {
                self.mesh.transform();
                self.renderer.draw_screen(&self.mesh, self.input.help);
            }

            if self.input.upd.enhance && self.expr.is_implicit {
                self.mesh.generate_screen(&self.expr);
            }

            self.input.update();
            self.timer.update();
            
            if self.timer.fps < 800.0 {
                header_info(&format!("fps: {:.1}   ", self.timer.fps));
            }
        }
    }
}
