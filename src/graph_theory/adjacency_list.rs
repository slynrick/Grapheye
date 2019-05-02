use std::fmt;
use super::graph::{Graph, GraphJson};
use std::collections::LinkedList;

use serde_json::{Result, Value, json};

#[derive(Serialize, Deserialize)]
pub struct AdjacencyList {
    m: u32,
    n: u32,
    L: Vec<LinkedList<u32>>,
}

impl AdjacencyList {
    pub fn new(m: u32, n: u32) -> AdjacencyList {
        AdjacencyList {
            m,
            n,
            L: vec![LinkedList::new(); n as usize]
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
    
    fn read_json(&mut self, data: String) {
        let json: GraphJson = serde_json::from_str(&data[..]).unwrap();
        for r in json.arestas.iter() {
            let node1 = r[0].parse::<usize>().unwrap();
            let node2 = r[1].parse::<usize>().unwrap();
            self.L[node1].push_back(node2 as u32);
            self.L[node2].push_back(node1 as u32);
        }
    }

    fn write_json(&self) -> String {
        let mut N = vec![String::new(); self.n as usize];
        let mut M = vec![Vec::new(); 0];
        for n in 0..self.n {
            N[n as usize] = (n+1).to_string();
            for r in self.L[n as usize].iter() {
                M.push(vec![(n+1).to_string(), (r+1).to_string()])
            }
        }
        json!({
            "nome": "RESPONSE-RUST-ROCKET",
            "vertices": N,
            "arestas": M
        }).to_string()
    }

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
