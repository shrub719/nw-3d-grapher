use crate::{
    input::parser::Expr
};

pub fn get_graph() -> Expr {
    Expr::new("x 2 ^ y 2 ^ + z 2 ^ + 4 x * sin + 4 y * sin + 4 z * sin + 1.11 -", false).unwrap()
}
