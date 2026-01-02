use crate::{
    grapher::Graph
};

pub fn get_graph() -> Graph {
    |x, y, z| x*y*z
}
