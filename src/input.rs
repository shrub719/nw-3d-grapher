use crate::{
    grapher::Grapher
};

pub mod parser;
pub mod ui;

pub fn main_loop() {
    loop {
        Grapher::setup_ui();
        let graph = match ui::get_expr() {
            Some(e) => e,
            None => break
        };
        let mut grapher = Grapher::new(graph);
        grapher.main_loop();
    }
}
