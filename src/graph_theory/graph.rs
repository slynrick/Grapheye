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

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchMatrix {
    nodes: u32,
    Visited: Vec<bool>,
    Explored : Vec< Vec<bool> >,
    Discovery : Vec< Vec<bool> >
}

impl SearchMatrix {
    pub fn new (nodes: u32) -> SearchMatrix {
        SearchMatrix {
            nodes,
            Visited: vec![false; nodes as usize],
            Explored : vec![vec![false; nodes as usize];  nodes as usize],
            Discovery : vec![vec![false; nodes as usize];  nodes as usize]
        }
    }

    pub fn add_node(&mut self) {
        self.nodes += 1;
        self.Visited.push(false);
        self.Explored.push(vec![false; self.nodes as usize]);
        for r in self.Explored.iter_mut() {
            r.push(false);
        }
        self.Discovery.push(vec![false; self.nodes as usize]);
        for r in self.Discovery.iter_mut() {
            r.push(false);
        }
    }

    pub fn rm_node(&mut self, node: u32) {
        self.nodes -=1;
        self.Visited.remove(node as usize);
        self.Explored.remove(node as usize);
        for r in self.Explored.iter_mut() {
            r.remove(node as usize);
        }
        self.Discovery.remove(node as usize);
        for r in self.Discovery.iter_mut() {
            r.remove(node as usize);
        }
    }

    pub fn set_visited(&mut self, node: u32) {
        self.Visited[node as usize] = true;
    }

    pub fn is_visited(&mut self, node: u32) -> bool {
        self.Visited[node as usize]
    }

    pub fn set_explored(&mut self, node1: u32, node2: u32) {
        self.Explored[node1 as usize][node2 as usize] = true;
        self.Explored[node2 as usize][node1 as usize] = true;
    }

    pub fn is_explored(&mut self, node1: u32, node2: u32) -> bool {
         self.Explored[node1 as usize][node2 as usize]
    }

    pub fn set_discovery(&mut self, node1: u32, node2: u32) {
        self.Discovery[node1 as usize][node2 as usize] = true;
        self.Discovery[node2 as usize][node1 as usize] = true;
    }

    pub fn is_discovery(&mut self, node1: u32, node2: u32) -> bool {
         self.Discovery[node1 as usize][node2 as usize]
    }
}

pub trait Graph {
    fn add_edge(&mut self, node1: u32, node2: u32, cost: u32);
    fn rm_edge(&mut self, node1: u32, node2: u32);
    fn add_node(&mut self);
    fn rm_node(&mut self, node: u32);
    fn get_neighborhood(&self, node: u32) -> Vec<u32>;
    fn search(&mut self, node: u32);
    fn shallow_search(&mut self);
    fn full_search(&mut self);
    fn is_connected(&mut self) -> bool;
    fn has_cycle(&mut self) -> bool;
    fn is_forest(&mut self) -> bool;
    fn is_tree(&mut self) -> bool;
    fn get_forest_generator(&mut self) -> GraphJson;
    fn deepfirst_search(&mut self, node: u32) -> Vec<Vec<String>> ;
    fn breadthfirst_search(&mut self, node: u32) -> Vec<Vec<String>> ;
    fn define_distances(&mut self, node: u32) -> Vec<i32>;
    fn dijkstra(&mut self, node: u32) -> (Vec<u32>, Vec<u32>);
}

