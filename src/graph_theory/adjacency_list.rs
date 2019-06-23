#![allow(non_snake_case)]
use std::fmt;
use super::graph::{Graph, GraphJson, SearchMatrix};
use std::collections::{LinkedList, VecDeque};

#[derive(Serialize, Deserialize)]
pub struct AdjacencyList {
    name: String,
    m: u32,
    n: u32,
    L: Vec<LinkedList<(u32, u32)>>, // (node, weight)
    was_fully_searched: bool,
    was_shallow_searched: bool,
    search_matrix: SearchMatrix
}


impl AdjacencyList {
    pub fn new(m: u32, n: u32, name: String) -> AdjacencyList {
        
        AdjacencyList {
            name,
            m,
            n,
            L: vec![LinkedList::new(); n as usize],
            was_fully_searched: false,
            was_shallow_searched: false,
            search_matrix: SearchMatrix::new(n)
        }
    }

    pub fn from_json(data: String) -> AdjacencyList {
        let json: GraphJson = serde_json::from_str(&data[..]).unwrap();
        let mut obj = AdjacencyList {
            name: json.nome,
            m: 0,
            n: json.vertices.len() as u32,
            L: vec![LinkedList::new(); json.vertices.len()],
            was_fully_searched: false,
            was_shallow_searched: false,
            search_matrix: SearchMatrix::new(json.vertices.len() as u32)
        };
        for r in json.arestas.iter() {
            let node1 = r[0].parse::<u32>().unwrap() - 1;
            let node2 = r[1].parse::<u32>().unwrap() - 1;
            if r.len() == 3 {
                obj.add_edge(node1, node2, r[2].parse::<u32>().unwrap());
            } else {
                obj.add_edge(node1, node2, u32::max_value());
            }
        }
        obj
    }

    pub fn get_GraphJson(&self) -> GraphJson {
        let mut nodes = vec![String::new(); self.n as usize];
        let mut edges = vec![Vec::new(); 0];
        for n in 0..self.n {
            nodes[n as usize] = (n+1).to_string();
            for r in self.L[n as usize].iter() {
                let (node1, node2) = if n > (*r).0 { ((*r).0 + 1, n + 1) } else { (n + 1, (*r).0 + 1) };
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

    pub fn get_SearchMatrix(&self) -> SearchMatrix {
        self.search_matrix.clone()
    }

    pub fn erase_search(&mut self) {
        self.search_matrix = SearchMatrix::new(self.n);
    }

}

impl fmt::Display for AdjacencyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialized = serde_json::to_string(&self).unwrap();
        write!(f, "{:#?}", serialized)
    }
}

impl Graph for AdjacencyList {

    fn add_edge(&mut self, node1: u32, node2: u32, cost: u32) {
        self.m += 1;
        self.L[node1 as usize].push_back((node2,cost));
        self.L[node2 as usize].push_back((node1,cost));
    }

    fn rm_edge(&mut self, node1: u32, node2: u32) {
        self.m -= 1;
        self.L[node1 as usize] = self.L[node1 as usize].iter()
                                    .filter(|x| (**x).0 != node2)
                                    .map(|x| *x)
                                    .collect();
        self.L[node2 as usize] = self.L[node2 as usize].iter()
                                    .filter(|x| (**x).0 != node1)
                                    .map(|x| *x)
                                    .collect();
    }

    fn add_node(&mut self) {
        self.n +=1;
        self.L.push(LinkedList::new());
        self.search_matrix.add_node();
        self.search(self.n - 1);
    }

    fn rm_node(&mut self, node: u32) {
        self.n -= 1;
        for r in 0..self.n {
            self.L[r as usize] = self.L[r as usize].iter()
                                    .filter(|x| (**x).0 != node)
                                    .map(|x| if (*x).0 < node { *x } else { ((*x).0 - 1, (*x).1) })
                                    .collect();
        }
        self.L.remove(node as usize);
        self.search_matrix.rm_node(node);
    }

    fn get_neighborhood(&self, node: u32) -> Vec<u32> {
        let mut neighborhood = Vec::new();
        for r in self.L[node as usize].iter() {
            neighborhood.push((*r).0);
        }
        neighborhood
    }

    fn search(&mut self, node: u32) {
        let mut selected_node = node;
        self.search_matrix.set_visited(selected_node);
        let mut nodes : Vec<u32> = Vec::new();
        loop {
            for r in self.L[selected_node as usize].iter() {
                if !self.search_matrix.is_explored(selected_node, (*r).0) {
                    self.search_matrix.set_explored(selected_node, (*r).0);
                    if !self.search_matrix.is_visited((*r).0) {
                        nodes.push((*r).0);
                        self.search_matrix.set_visited((*r).0);
                        self.search_matrix.set_discovery(selected_node, (*r).0);
                    }
                }
            }

            if nodes.len() <= 0 {
                break;
            }

            selected_node = nodes.pop().unwrap();
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
            for r in self.L[n as usize].iter() {
                if !self.search_matrix.is_discovery(n, (*r).0) {
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
            for r in self.L[n as usize].iter() {
                let (node1, node2) = if n > (*r).0 { ((*r).0 + 1, n + 1) } else { (n + 1, (*r).0 + 1) };
                if !edges.iter().any(|x| x[0] == node1.to_string() && x[1] == node2.to_string()) {
                    if self.search_matrix.is_discovery(n, (*r).0) {
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

    fn deepfirst_search(&mut self, node: u32) -> Vec<Vec<String>>  {
        let mut stages = vec![Vec::new(); 0];

        self.search_matrix.set_visited(node);

        let mut neighborhood : LinkedList<u32> = self.L[node as usize].iter()
                                    .map(|x| (*x).0)
                                    .collect();

        for w in neighborhood.iter_mut() {
            if self.search_matrix.is_visited(*w) {
                if !self.search_matrix.is_explored(node, *w) {
                   self.search_matrix.set_explored(node, *w);
                }
            } else {
                self.search_matrix.set_explored(node, *w);
                self.search_matrix.set_discovery(node, *w);
                stages.push(vec![(node + 1).to_string(), (*w + 1).to_string()]);
                stages.append(&mut self.deepfirst_search(*w));
            }
        }

        stages
    }

    fn breadthfirst_search(&mut self, node: u32) -> Vec<Vec<String>> {
        let mut stages = vec![Vec::new(); 0];

        if self.was_fully_searched {
            self.search_matrix = SearchMatrix::new(self.n);
        }
        let mut F : VecDeque<u32> = VecDeque::new();
        self.search_matrix.set_visited(node);
        F.push_back(node);
        while F.len() > 0 {
            let v = F.pop_front().unwrap();

            for w in self.L[v as usize].iter() {
                if self.search_matrix.is_visited((*w).0) {
                    if !self.search_matrix.is_explored(v, (*w).0) {
                        self.search_matrix.set_explored(v, (*w).0);
                    }
                } else {
                    self.search_matrix.set_explored(v, (*w).0);
                    self.search_matrix.set_discovery(v, (*w).0);
                    self.search_matrix.set_visited((*w).0);
                    stages.push(vec![(v + 1).to_string(), ((*w).0 + 1).to_string()]);
                    F.push_back((*w).0);
                }
            }
        }

        stages
    }

    fn define_distances(&mut self, node: u32) -> Vec<i32> {
        if self.was_fully_searched {
            self.search_matrix = SearchMatrix::new(self.n);
        }
        let mut F : VecDeque<(u32, i32)> = VecDeque::new();
        let mut Dist = vec![-1; self.n as usize];
        self.search_matrix.set_visited(node);
        F.push_back((node, 1));
        while F.len() > 0 {
            let (v, lvl) = F.pop_front().unwrap();

            for w in self.L[v as usize].iter() {
                if self.search_matrix.is_visited((*w).0) {
                    if !self.search_matrix.is_explored(v, (*w).0) {
                        self.search_matrix.set_explored(v, (*w).0);
                    }
                } else {
                    self.search_matrix.set_explored(v, (*w).0);
                    self.search_matrix.set_discovery(v, (*w).0);
                    self.search_matrix.set_visited((*w).0);
                    Dist[(*w).0 as usize] = lvl;
                    F.push_back(((*w).0, lvl+1));
                }
            }
        }

        Dist
    }

    fn dijkstra(&mut self, node: u32) -> (Vec<u32>, Vec<u32>) {
        let mut d = vec![u32::max_value(); self.n as usize];
        let mut T = vec![false; self.n as usize];
        let mut P = vec![u32::max_value(); self.n as usize];

        d[node as usize] = 0;
        for i in 1..self.n {
            let mut u = 0;
            let mut min_cost = u32::max_value();
            for n in 0..T.len() {
                for (sel, cost) in self.L[n as usize].iter() {
                    if !T[n] && cost < min_cost {
                        u = n;
                        min_cost = cost;
                    }
                }
                
            }
            T[u] = true;

            for (v, cost) in self.L[u].iter() {
                if d[*v as usize] > d[u] + cost {
                    d[*v as usize] = d[u] + cost;
                    P[*v as usize] = u as u32;
                }
            }
        }
        (d, P)
    }
}
