
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(duration_float)]
#![feature(rustc_private)]
#[macro_use] extern crate rocket;
extern crate rand;

use rocket_cors;
use std::time::Instant;
use rand::Rng;
use serde_json::json;
use rust_graph_theory::graph_theory::graph::Graph;
use rust_graph_theory::graph_theory::adjacency_list::AdjacencyList;
use rust_graph_theory::graph_theory::adjacency_matrix::AdjacencyMatrix;
use rust_graph_theory::graph_theory::graph::GraphJson;


#[post("/read/<method>", data = "<var>")]
fn read(method: String, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        "AdjacencyMatrix" => {
          let x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/edge/add/<method>", data = "<var>")]
fn add_edge(method: String, var: String) -> String {
    let start = Instant::now();
    let json: GraphJson = serde_json::from_str(&var[..]).unwrap();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            for r in json.par_arestas.iter() {
                let node1 = r[0].parse::<u32>().unwrap() - 1;
                let node2 = r[1].parse::<u32>().unwrap() - 1;
                if r.len() == 3 {
                    x.add_edge(node1, node2, r[2].parse::<u32>().unwrap());
                } else {
                    x.add_edge(node1, node2, u32::max_value());
                }
            } 
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        "AdjacencyMatrix" => {
           let mut x = AdjacencyMatrix::from_json(var);
            for r in json.par_arestas.iter() {
                let node1 = r[0].parse::<u32>().unwrap() - 1;
                let node2 = r[1].parse::<u32>().unwrap() - 1;
                if r.len() == 3 {
                    x.add_edge(node1, node2, r[2].parse::<u32>().unwrap());
                } else {
                    x.add_edge(node1, node2, u32::max_value());
                }
            }
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/edge/remove/<method>", data = "<var>")]
fn rm_edge(method: String, var: String) -> String {
    let start = Instant::now();
    let json: GraphJson = serde_json::from_str(&var[..]).unwrap();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            for r in json.par_arestas.iter() {
                let node1 = r[0].parse::<u32>().unwrap() - 1;
                let node2 = r[1].parse::<u32>().unwrap() - 1;
                x.rm_edge(node1, node2);
            } 
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            for r in json.par_arestas.iter() {
                let node1 = r[0].parse::<u32>().unwrap() - 1;
                let node2 = r[1].parse::<u32>().unwrap() - 1;
                x.rm_edge(node1, node2);
            }
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/node/add/<method>", data = "<var>")]
fn add_node(method: String, var: String) -> String {
    let start = Instant::now();
    let json: GraphJson = serde_json::from_str(&var[..]).unwrap();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            for _r in json.par_vertices.iter() {
                //let node = r.parse::<u32>().unwrap() - 1;
                x.add_node();
            } 
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            for _r in json.par_vertices.iter() {
                //let node = r.parse::<u32>().unwrap() - 1;
                x.add_node();
            }
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/node/remove/<method>", data = "<var>")]
fn rm_node(method: String, var: String) -> String {
    let start = Instant::now();
    let json: GraphJson = serde_json::from_str(&var[..]).unwrap();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            for r in json.par_vertices.iter() {
                let node = r.parse::<u32>().unwrap() - 1;
                x.rm_node(node);
            } 
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            for r in json.par_vertices.iter() {
                let node = r.parse::<u32>().unwrap() - 1;
                x.rm_node(node);
            }
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/node/neighborhood/<method>/<node>", data = "<var>")]
fn neighborhood(method: String, node: u32, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let x = AdjacencyList::from_json(var);
            let nh : Vec<u32> = x.get_neighborhood(node-1).iter().map(|x| x+1).collect();
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": nh }).to_string()
        },
        "AdjacencyMatrix" => {
            let x = AdjacencyMatrix::from_json(var);
            let nh : Vec<u32> = x.get_neighborhood(node-1).iter().map(|x| x+1).collect();
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": nh }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}


#[get("/generate/<nodes>")]
fn generate_random_graphs(nodes: u32) -> String {
    let mut rng = rand::thread_rng();
    let edges = rng.gen::<u32>() % ( nodes*(nodes-1)/2 + 1) ;
    let mut nodes_v = vec![String::new(); nodes as usize];
    let mut edges_v = vec![Vec::new(); 0];
    for n in 0..nodes {
        nodes_v[n as usize] = (n+1).to_string();
    }
    for _r in 0..edges {
        let tmp1 = rng.gen::<u32>() % nodes;
        let tmp2 = rng.gen::<u32>() % nodes;
        let (node1, node2) = if tmp1 > tmp2 { (tmp2 + 1, tmp1 + 1) } else { (tmp1 + 1, tmp2 + 1) };
        if !edges_v.iter().any(|x| x[0] == node1.to_string() && x[1] == node2.to_string()) && node1 != node2 {
            edges_v.push(vec![node1.to_string(), node2.to_string()]);
        }
    }
    let new_graph = GraphJson {
        nome: format!("GRAFO_ALEATORIO_N{}_M{}",
               nodes_v.len(), edges_v.len()),
        vertices: nodes_v,
        arestas: edges_v,
        par_vertices: Vec::new(),
        par_arestas: Vec::new()
    };
    json!(new_graph).to_string()
}

#[post("/search/<search_type>/<method>", data = "<var>")]
fn search(search_type: String, method: String, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut status : String = "OK".to_string();
            let mut x = AdjacencyList::from_json(var);
            match search_type.as_ref() {
                "full" => x.full_search(),
                "shallow" => x.shallow_search(),
                _ => status = "FAIL".to_string(),
            }
        
            json!({ "status": status, "duration": start.elapsed().as_secs_f64(), "data": x.get_SearchMatrix() }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut status : String = "OK".to_string();
            let mut x = AdjacencyMatrix::from_json(var);
            match search_type.as_ref() {
                "full" => x.full_search(),
                "shallow" => x.shallow_search(),
                _ => status = "FAIL".to_string(),
            }
        
            json!({ "status": status, "duration": start.elapsed().as_secs_f64(), "data": x.get_SearchMatrix() }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/is_connected/<method>", data = "<var>")]
fn is_connected(method: String, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "is_connected": x.is_connected(), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "is_connected": x.is_connected(), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/has_cycle/<method>", data = "<var>")]
fn has_cycle(method: String, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "has_cycle": x.has_cycle(), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "has_cycle": x.has_cycle(), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/is_forest/<method>", data = "<var>")]
fn is_forest(method: String, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "is_forest": x.is_forest(), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "is_forest": x.is_forest(), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/is_tree/<method>", data = "<var>")]
fn is_tree(method: String, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "is_tree": x.is_tree(), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "is_tree": x.is_tree(), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/get_forest_generator/<method>", data = "<var>")]
fn get_forest_generator(method: String, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_forest_generator() }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_forest_generator() }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/deepfirst_search/<method>/<node>", data = "<var>")]
fn deepfirst_search(method: String, node: u32, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "stages": x.deepfirst_search(node), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "stages": x.deepfirst_search(node), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/breadthfirst_search/<method>/<node>", data = "<var>")]
fn breadthfirst_search(method: String, node: u32, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "stages": x.breadthfirst_search(node), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": { "stages": x.breadthfirst_search(node), "search_matrix": x.get_SearchMatrix() } }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}


#[post("/define_distances/<method>/<node>", data = "<var>")]
fn define_distances(method: String, node: u32, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.define_distances(node) }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.define_distances(node) }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

#[post("/dijkstra/<method>/<node>", data = "<var>")]
fn dijkstra(method: String, node: u32, var: String) -> String {
    let start = Instant::now();
    match method.as_ref() {
        "AdjacencyList" => {
            let mut x = AdjacencyList::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.dijkstra(node) }).to_string()
        },
        "AdjacencyMatrix" => {
            let mut x = AdjacencyMatrix::from_json(var);
            json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.dijkstra(node) }).to_string()
        },
        _ => json!({ "status": "FAIL"}).to_string(),
    }
}

fn main() {
    let cors = rocket_cors::Cors::default();
    
    rocket::ignite()
        .mount("/api", routes![read])
        .mount("/api/exec", routes![add_edge, rm_edge, 
                                    add_node, rm_node, 
                                    neighborhood, 
                                    generate_random_graphs,
                                    search,
                                    is_connected,
                                    has_cycle,
                                    is_forest,
                                    is_tree,
                                    get_forest_generator,
                                    deepfirst_search,
                                    breadthfirst_search,
                                    define_distances,
                                    dijkstra])
        .attach(cors)
    .launch();
}