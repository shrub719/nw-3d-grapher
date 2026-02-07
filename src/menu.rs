use crate::{
    grapher::Grapher
};

pub mod parser;
pub mod ui;

pub fn main_loop() {
    loop {
        let graph = ui::get_graph();
        let mut grapher = Grapher::new(graph);
        grapher.main_loop();
    }
}
