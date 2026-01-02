use crate::{
    grapher::Grapher
};

pub mod parser;
pub mod ui;

pub enum MenuState {
    Input,
    Settings
}

pub struct Menu {
    state: MenuState
}
impl Menu {
    pub fn new() -> Self {
        Menu {
            state: MenuState::Input
        }
    }
    
    pub fn main_loop(&self) {
        loop {
            let graph = ui::get_graph();
            let mut grapher = Grapher::new(graph);
            grapher.main_loop();
        }
    }
}
