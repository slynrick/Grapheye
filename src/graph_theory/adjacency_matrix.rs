#![allow(non_snake_case)]
#![allow(dead_code)]
use std::fmt;
use super::graph::{Graph, GraphJson, SearchMatrix};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize)]
pub struct AdjacencyMatrix {
    name: String,
    m: u32,
    n: u32,
    M : Vec< Vec<bool> >,
    was_fully_searched: bool,
    was_shallow_searched: bool,
    search_matrix: SearchMatrix
}

impl AdjacencyMatrix {
    fn new(m: u32, n: u32, name: String) -> AdjacencyMatrix {
        AdjacencyMatrix {
            name,
            m,
            n,
            M : vec![vec![false; n as usize]; n as usize],
            was_fully_searched: false,
            was_shallow_searched: false,
            search_matrix: SearchMatrix::new(n)
        }
    }

    pub fn from_json(data: String) -> AdjacencyMatrix {
        let json: GraphJson = serde_json::from_str(&data[..]).unwrap();
        let mut obj = AdjacencyMatrix {
            name: json.nome,
            m: 0,
            n: json.vertices.len() as u32,
            M : vec![vec![false; json.vertices.len()]; json.vertices.len()],
            was_fully_searched: false,
            was_shallow_searched: false,
            search_matrix: SearchMatrix::new(json.vertices.len() as u32)
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

    fn search(&mut self, node: u32) {
        self.search_matrix.set_visited(node);
        for r in 0..self.M[node as usize].len() {
            for w in 0..self.M[r].len()  {
                if !self.M[r][w] {
                    continue;
                }
                if self.search_matrix.is_visited(r as u32) && 
                !self.search_matrix.is_explored(w as u32, r as u32) {
                    self.search_matrix.set_explored(w as u32, r as u32);
                    if !self.search_matrix.is_visited(w as u32) {
                        self.search_matrix.set_visited(w as u32);
                        self.search_matrix.set_discovery(w as u32, r as u32);
                    }
                }
            }
        }
    }

    fn shallow_search(&mut self) {
        self.search(0);
        self.was_shallow_searched = true;
    }

    fn full_search(&mut self) {
        for r in 0..self.n {
            if !self.search_matrix.is_visited(r) {
                self.search(r);
            }
        }
        self.was_fully_searched = true;
    }

    fn is_connected(&mut self) -> bool {
        if !self.was_shallow_searched {
            self.shallow_search();
        }
        for n in 0..self.n {
            if !self.search_matrix.is_visited(n) {
                return false
            }
        }
        true
    }

    fn has_cycle(&mut self) -> bool {
        if !self.was_fully_searched {
            self.full_search();
        }
        for n in 0..self.n {
            for r in 0..self.M[n as usize].len() {
                if !self.M[n as usize][r] {
                    continue;
                }
                if !self.search_matrix.is_discovery(n, r as u32) {
                    return true
                }
            }
        }
        false
    }

    fn is_forest(&mut self) -> bool {
        !self.has_cycle()
    }

    fn is_tree(&mut self) -> bool {
        self.is_connected() && !self.has_cycle()
    }

    fn get_forest_generator(&mut self) -> GraphJson {
        if !self.was_fully_searched {
            self.full_search();
        }

        let mut nodes = vec![String::new(); self.n as usize];
        let mut edges = vec![Vec::new(); 0];

        for n in 0..self.n {
            nodes[n as usize] = (n+1).to_string();
            for r in 0..self.M[n as usize].len() {
                if !self.M[n as usize][r as usize] {
                    continue;
                }
                let (node1, node2) = if n > (r as u32) { ((r + 1) as u32, (n + 1) as u32) } else { ( (n + 1) as u32, (r + 1) as u32) };
                if !edges.iter().any(|x| x[0] == node1.to_string() && x[1] == node2.to_string()) {
                    if self.search_matrix.is_discovery(node1, node2) {
                        edges.push(vec![node1.to_string(), node2.to_string()])
                    }
                }
            }
        }
        self.name.push_str("-ArvoreGeradora");
        GraphJson {
            nome: (*self.name).to_string(),
            vertices: nodes,
            arestas: edges,
            par_vertices: Vec::new(),
            par_arestas: Vec::new()
        }
    }

    fn deepfirst_search(&mut self, node: u32) {
        if self.was_fully_searched {
            self.search_matrix = SearchMatrix::new(self.n);
        }
        self.search_matrix.set_visited(node);

        for w in 0..self.M[node as usize].len() {
            if !self.M[node as usize][w] {
                continue;
            }
            if self.search_matrix.is_visited(w as u32) {
                if !self.search_matrix.is_explored(node, w as u32) {
                   self.search_matrix.set_explored(node, w as u32);
                }
            } else {
                self.search_matrix.set_explored(node, w as u32);
                self.search_matrix.set_discovery(node, w as u32);
                self.deepfirst_search(w as u32);
            }
        }
    }

    fn breadthfirst_search(&mut self, node: u32) {
        if self.was_fully_searched {
            self.search_matrix = SearchMatrix::new(self.n);
        }
        let mut F : VecDeque<u32> = VecDeque::new();
        self.search_matrix.set_visited(node);
        F.push_back(node);
        while F.len() > 0 {
            let v = F.pop_front().unwrap();

            for w in 0..self.M[v as usize].len() {
                if !self.M[v as usize][w] {
                    continue;
                }
                if self.search_matrix.is_visited(w as u32) {
                    if !self.search_matrix.is_explored(v, w as u32) {
                        self.search_matrix.set_explored(v, w as u32);
                    }
                } else {
                    self.search_matrix.set_explored(v, w as u32);
                    self.search_matrix.set_discovery(v, w as u32);
                    self.search_matrix.set_visited(w as u32);
                    F.push_back(w as u32);
                }
            }
        }
    }

    fn define_distances(&mut self, node: u32) {
        if self.was_fully_searched {
            self.search_matrix = SearchMatrix::new(self.n);
        }
        let mut F : VecDeque<(u32, i32)> = VecDeque::new();
        let mut Dist = vec![-1; self.n as usize];
        self.search_matrix.set_visited(node);
        F.push_back((node, 1));
        while F.len() > 0 {
            let (v, lvl) = F.pop_front().unwrap();

            for w in 0..self.M[v as usize].len() {
                if !self.M[v as usize][w] {
                    continue;
                }
                if self.search_matrix.is_visited(w as u32) {
                    if !self.search_matrix.is_explored(v, w as u32) {
                        self.search_matrix.set_explored(v, w as u32);
                    }
                } else {
                    self.search_matrix.set_explored(v, w as u32);
                    self.search_matrix.set_discovery(v, w as u32);
                    self.search_matrix.set_visited(w as u32);
                    Dist[w] = lvl;
                    F.push_back((w as u32, lvl+1));
                }
            }
        }
    }
}
