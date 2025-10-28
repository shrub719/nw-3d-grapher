use crate::{
    grapher::{
        renderer::*,
        mesh::Mesh,
        input::*,
        timer::*
    },
    eadk::*,
    constants::{ graphics::*, palette::* }
};
#[cfg(target_os = "none")]
use alloc::format;

pub mod mat;
mod renderer;
mod mesh;
mod input;
mod timer;

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

    fn setup_ui() {
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

        self.mesh.domain.update_matrix();
        self.mesh.load_mesh_from_file();

        // main loop - runs every frame
        while !self.input.upd.quit {   
            if self.input.upd.rotation {
                self.mesh.update_rotation(self.input.rotation_direction, self.timer.delta_time);
            }
            if self.input.upd.scale {
                self.mesh.update_scale(self.input.scale_change, self.timer.delta_time);
            }

            if self.input.upd.redraw {
                self.mesh.transform();
                self.renderer.draw_screen(&self.mesh, self.input.shading);
            }

            self.input.update();
            self.timer.update();
            
            if self.timer.fps < 800.0 {   // temp fix
                header_info(&format!("fps: {:.1}   ", self.timer.fps));
            }
        }
    }
}