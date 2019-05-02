use std::fmt;
use super::graph::{Graph, GraphJson};
use serde_json::{Result, Value, json};

#[derive(Serialize, Deserialize)]
pub struct AdjacencyMatrix {
    m: u32,
    n: u32,
    M : Vec< Vec<bool> >
}

impl AdjacencyMatrix {
    fn new(m: u32, n: u32) -> AdjacencyMatrix {
        AdjacencyMatrix {
            m,
            n,
            M : vec![vec![false; n as usize]; n as usize]
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

    fn read_json(&mut self, data: String) {
        let json: GraphJson = serde_json::from_str(&data[..]).unwrap();
        for r in json.arestas.iter() {
            let node1 = r[0].parse::<usize>().unwrap();
            let node2 = r[1].parse::<usize>().unwrap();
            self.M[node1][node2] = true;
            self.M[node2][node1] = true;
        }
    }

    fn write_json(&self) -> String {
        let mut N = vec![String::new(); self.n as usize];
        let mut M = vec![Vec::new(); 0];
        for n in 0..self.n {
            N[n as usize] = (n+1).to_string();
            for r in self.M[n as usize].iter()
                                        .enumerate()
                                        .filter(|data| *data.1 == true)
                                        .map(|data| data.0 as u32) {
                M.push(vec![(r+1).to_string(), (n+1).to_string()])
            }
        }
        json!({
            "nome": "RESPONSE-RUST-ROCKET",
            "vertices": N,
            "arestas": M
        }).to_string()
    }

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
