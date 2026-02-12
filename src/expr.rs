use crate::{
    grapher::Grapher,
    eadk::header_info
};

pub mod parser;
pub mod ui;

pub fn main_loop() {
    loop {
        Grapher::setup_ui();
        header_info("input");

        let graph = match ui::get_expr() {
            Some(e) => e,
            None => break
        };
        let mut grapher = Grapher::new(graph);
        grapher.main_loop();
    }
}
