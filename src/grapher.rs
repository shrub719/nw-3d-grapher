use crate::{
    renderer::*,
    config::*,
    mesh::*,
    mat::*,
    eadk
};

pub struct Grapher {
    renderer: Renderer,
    mesh: Mesh
}
impl Grapher {
    pub fn new() -> Self {
        Grapher {
            renderer: Renderer::new(),
            mesh: Mesh::new(),
        }
    }

    pub fn main_loop(&mut self) {
        let scale = 2.0;
        let matrix = Matrix3 ( [
            [scale, 0.0, 0.0],
            [0.0, scale, 0.0],
            [0.0, 0.0, 1.0]
        ] );


        // main loop - runs every frame
        let mut keyboard_state: eadk::input::KeyboardState = eadk::input::KeyboardState::scan();
        while !keyboard_state.key_down(eadk::input::Key::Back) {
            let color = eadk::Color::from_rgb(eadk::random() as u16, eadk::random() as u16, eadk::random() as u16);
            self.renderer.draw_screen(&self.mesh, color);

            keyboard_state = eadk::input::KeyboardState::scan();
            while !(keyboard_state.key_down(eadk::input::Key::Ok) || keyboard_state.key_down(eadk::input::Key::Back)) {
                keyboard_state = eadk::input::KeyboardState::scan();
                eadk::timing::msleep(50);
            }
        }
    }
}