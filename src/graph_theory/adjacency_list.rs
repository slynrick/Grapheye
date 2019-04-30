use std::fmt;
use super::graph::Graph;

pub struct Node {
    num_neighborhood: u32,
    
}

pub struct AdjacencyList {
}

impl AdjacencyList {
}

impl fmt::Display for AdjacencyList {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "AdjacencyList")
    }
}

impl Graph for AdjacencyList {
    fn new(m: u32, n: u32) -> AdjacencyList {
        AdjacencyList {}
    }

    fn add_edge(&mut self, node1: u32, node2: u32) {
    }

    fn rm_edge(&mut self, node1: u32, node2: u32) {
    }

    fn add_node(&mut self) {
    }

    fn rm_node(&mut self, node: u32) {
    }

    fn get_neighborhood(&self, node: u32) -> Vec<u32> {
        vec![0; 0]
    }
}
