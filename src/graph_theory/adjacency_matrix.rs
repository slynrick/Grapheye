#![allow(non_snake_case)]
#![allow(dead_code)]
use std::fmt;
use super::graph::{Graph, GraphJson};

#[derive(Serialize, Deserialize)]
pub struct AdjacencyMatrix {
    name: String,
    m: u32,
    n: u32,
    M : Vec< Vec<bool> >
}

impl AdjacencyMatrix {
    fn new(m: u32, n: u32, name: String) -> AdjacencyMatrix {
        AdjacencyMatrix {
            name,
            m,
            n,
            M : vec![vec![false; n as usize]; n as usize]
        }
    }

    pub fn from_json(data: String) -> AdjacencyMatrix {
        let json: GraphJson = serde_json::from_str(&data[..]).unwrap();
        let mut obj = AdjacencyMatrix {
            name: json.nome,
            m: 0,
            n: json.vertices.len() as u32,
            M : vec![vec![false; json.vertices.len()]; json.vertices.len()]
        };
        for r in json.arestas.iter() {
            let node1 = r[0].parse::<u32>().unwrap() - 1;
            let node2 = r[1].parse::<u32>().unwrap() - 1;
            obj.add_edge(node1, node2);
        }
        obj
    }

    pub fn get_GraphJson(&self) -> GraphJson {
        let mut nodes = vec![String::new(); self.n as usize];
        let mut edges = vec![Vec::new(); 0];
        for n in 0..self.n {
            nodes[n as usize] = (n+1).to_string();
            for r in self.M[n as usize].iter()
                                        .enumerate()
                                        .filter(|data| *data.1 == true)
                                        .map(|data| data.0 as u32) {
                let (node1, node2) = if n > r { (r + 1, n + 1) } else { (n + 1, r + 1) };
                if !edges.iter().any(|x| x[0] == node1.to_string() && x[1] == node2.to_string()) {
                    edges.push(vec![node1.to_string(), node2.to_string()])
                }
            }
        }
        GraphJson {
            nome: (*self.name).to_string(),
            vertices: nodes,
            arestas: edges,
            par_vertices: Vec::new(),
            par_arestas: Vec::new()
        }
    }
}

impl fmt::Display for AdjacencyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialized = serde_json::to_string(&self).unwrap();
        write!(f, "{:#?}", serialized)
    }
}

impl Graph for AdjacencyMatrix {

    fn add_edge(&mut self, node1: u32, node2: u32) {
        self.M[node1 as usize][node2 as usize] = true;
        self.M[node2 as usize][node1 as usize] = true;
        self.m += 1;
    }

    fn rm_edge(&mut self, node1: u32, node2: u32) {
        self.M[node1 as usize][node2 as usize] = false;
        self.M[node2 as usize][node1 as usize] = false;
        self.m -=1;
    }

    fn add_node(&mut self) {
        self.n +=1;
        self.M.push(vec![false; self.n as usize]);
        for r in self.M.iter_mut() {
            r.push(false);
        }
    }

    fn rm_node(&mut self, node: u32) {
        self.n -=1;
        self.M.remove(node as usize);
        for r in self.M.iter_mut() {
            r.remove(node as usize);
        }
    }

    fn get_neighborhood(&self, node: u32) -> Vec<u32> {
        self.M[node as usize].iter()
            .enumerate()
            .filter(|data| *data.1 == true)
            .map(|data| data.0 as u32).collect()
    }
}
