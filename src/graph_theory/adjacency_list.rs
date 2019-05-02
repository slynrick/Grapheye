use std::fmt;
use super::graph::{Graph, GraphJson};
use std::collections::LinkedList;

#[derive(Serialize, Deserialize)]
pub struct AdjacencyList {
    name: String,
    m: u32,
    n: u32,
    L: Vec<LinkedList<u32>>,
}

impl AdjacencyList {
    pub fn new(m: u32, n: u32, name: String) -> AdjacencyList {
        AdjacencyList {
            name,
            m,
            n,
            L: vec![LinkedList::new(); n as usize]
        }
    }

    pub fn from_json(data: String) -> AdjacencyList {
        let json: GraphJson = serde_json::from_str(&data[..]).unwrap();
        let mut obj = AdjacencyList {
            name: json.nome,
            m: 0,
            n: json.vertices.len() as u32,
            L: vec![LinkedList::new(); json.vertices.len()]
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
            for r in self.L[n as usize].iter() {
                let (node1, node2) = if n > *r { (*r + 1, n + 1) } else { (n + 1, *r + 1) };
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

impl fmt::Display for AdjacencyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialized = serde_json::to_string(&self).unwrap();
        write!(f, "{:#?}", serialized)
    }
}

impl Graph for AdjacencyList {

    fn add_edge(&mut self, node1: u32, node2: u32) {
        self.m += 1;
        self.L[node1 as usize].push_back(node2);
        self.L[node2 as usize].push_back(node1);
    }

    fn rm_edge(&mut self, node1: u32, node2: u32) {
        self.m -= 1;
        self.L[node1 as usize] = self.L[node1 as usize].iter()
                                    .filter(|x| **x != node2)
                                    .map(|x| *x)
                                    .collect();
        self.L[node2 as usize] = self.L[node2 as usize].iter()
                                    .filter(|x| **x != node1)
                                    .map(|x| *x)
                                    .collect();
    }

    fn add_node(&mut self) {
        self.n +=1;
        self.L.push(LinkedList::new());
    }

    fn rm_node(&mut self, node: u32) {
        self.n -= 1;
        for r in 0..self.n {
            self.L[r as usize] = self.L[r as usize].iter()
                                    .filter(|x| **x != node)
                                    .map(|x| if *x < node { *x } else { *x - 1 })
                                    .collect()
        }
        self.L.remove(node as usize);
    }

    fn get_neighborhood(&self, node: u32) -> Vec<u32> {
        let mut neighborhood = Vec::new();
        for r in self.L[node as usize].iter() {
            neighborhood.push(*r);
        }
        neighborhood
    }
}
