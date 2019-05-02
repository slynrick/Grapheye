#[derive(Serialize, Deserialize)]
pub struct GraphJson {
    pub nome: String,
    pub vertices: Vec<String>,
    pub arestas: Vec<Vec<String>>
}

pub trait Graph {
    fn read_json(&mut self, data: String);
    fn write_json(&self) -> String;
    fn add_edge(&mut self, node1: u32, node2: u32);
    fn rm_edge(&mut self, node1: u32, node2: u32);
    fn add_node(&mut self);
    fn rm_node(&mut self, node: u32);
    fn get_neighborhood(&self, node: u32) -> Vec<u32>;
}

