pub trait Graph {
    fn new(m: u32, n: u32) -> Self;
    fn add_edge(&mut self, node1: u32, node2: u32);
    fn rm_edge(&mut self, node1: u32, node2: u32);
    fn add_node(&mut self);
    fn rm_node(&mut self, node: u32);
    fn get_neighborhood(&self, node: u32) -> Vec<u32>;
}

