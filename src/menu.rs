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
        // temp
        loop {
            let mut grapher = Grapher::new();
            grapher.main_loop();
        }
    }
}
