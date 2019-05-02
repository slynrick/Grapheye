#[derive(Serialize, Deserialize)]
pub struct GraphJson {
    pub nome: String,
    pub vertices: Vec<String>,
    pub arestas: Vec<Vec<String>>,

    #[serde(skip_serializing)]
    #[serde(default = "default_par_vertices")]
    pub par_vertices: Vec<String>,

    #[serde(skip_serializing)]
    #[serde(default = "default_par_arestas")]
    pub par_arestas: Vec<Vec<String>>
}

fn default_par_vertices() -> Vec<String> {
    Vec::new()
}

fn default_par_arestas() -> Vec<Vec<String>> {
    Vec::new()
}

pub trait Graph {
    fn add_edge(&mut self, node1: u32, node2: u32);
    fn rm_edge(&mut self, node1: u32, node2: u32);
    fn add_node(&mut self);
    fn rm_node(&mut self, node: u32);
    fn get_neighborhood(&self, node: u32) -> Vec<u32>;
}

