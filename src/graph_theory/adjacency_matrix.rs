use std::fmt;
use super::graph::Graph;

pub struct AdjacencyMatrix {
    m: u32,
    n: u32,
    M : Vec< Vec<bool> >
}

impl AdjacencyMatrix {
}

impl fmt::Display for AdjacencyMatrix {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "AdjacencyMatrix")
    }
}

impl Graph for AdjacencyMatrix {
    fn new(m: u32, n: u32) -> AdjacencyMatrix {
        AdjacencyMatrix {
            m,
            n,
            M : vec![vec![false; m as usize]; n as usize]
        }
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
        for r in 0..self.n {
            self.M[r as usize].push(false);
        }
    }

    fn rm_node(&mut self, node: u32) {
        self.n -=1;
        self.M.remove(node as usize);
        for r in 0..self.n {
            self.M[r as usize].remove(node as usize);
        }
    }

    fn get_neighborhood(&self, node: u32) -> Vec<u32> {
        let enumerated = self.M[node as usize].iter().enumerate().collect::<Vec<(usize, &bool)>>();
        let filtered = enumerated.iter().filter(|data| *data.1 == true).collect::<Vec<&(usize, &bool)>>();
        filtered.iter().map(|data| data.0 as u32).collect::<Vec<u32>>()
    }
}
