
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(duration_float)]
#![feature(rustc_private)]
#[macro_use] extern crate rocket;
extern crate rand;

use std::io;
use std::path::{Path, PathBuf};
use std::time::Instant;
use rand::Rng;
use rocket::response::NamedFile;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use serde_json::json;
use rust_graph_theory::graph_theory::graph::Graph;
use rust_graph_theory::graph_theory::adjacency_list::AdjacencyList;
use rust_graph_theory::graph_theory::adjacency_matrix::AdjacencyMatrix;
use rust_graph_theory::graph_theory::graph::GraphJson;

// API

#[post("/read/<method>", data = "<var>")]
fn read(method: String, var: String) -> String {
    let start = Instant::now();
    if method == "AdjacencyList" {
        let x = AdjacencyList::from_json(var);
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else if method == "AdjacencyMatrix"{
        let x = AdjacencyMatrix::from_json(var);
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else {
        json!({ "status": "FAIL"}).to_string()
    }
}

#[post("/edge/add/<method>", data = "<var>")]
fn add_edge(method: String, var: String) -> String {
    let start = Instant::now();
    let json: GraphJson = serde_json::from_str(&var[..]).unwrap();
    if method == "AdjacencyList" {
        let mut x = AdjacencyList::from_json(var);
        for r in json.par_arestas.iter() {
            let node1 = r[0].parse::<u32>().unwrap() - 1;
            let node2 = r[1].parse::<u32>().unwrap() - 1;
            x.add_edge(node1, node2);
        } 
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else if method == "AdjacencyMatrix"{
        let mut x = AdjacencyMatrix::from_json(var);
        for r in json.par_arestas.iter() {
            let node1 = r[0].parse::<u32>().unwrap() - 1;
            let node2 = r[1].parse::<u32>().unwrap() - 1;
            x.add_edge(node1, node2);
        }
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else {
        json!({ "status": "FAIL"}).to_string()
    }
}

#[post("/edge/remove/<method>", data = "<var>")]
fn rm_edge(method: String, var: String) -> String {
    let start = Instant::now();
    let json: GraphJson = serde_json::from_str(&var[..]).unwrap();
    if method == "AdjacencyList" {
        let mut x = AdjacencyList::from_json(var);
        for r in json.par_arestas.iter() {
            let node1 = r[0].parse::<u32>().unwrap() - 1;
            let node2 = r[1].parse::<u32>().unwrap() - 1;
            x.rm_edge(node1, node2);
        } 
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else if method == "AdjacencyMatrix"{
        let mut x = AdjacencyMatrix::from_json(var);
        for r in json.par_arestas.iter() {
            let node1 = r[0].parse::<u32>().unwrap() - 1;
            let node2 = r[1].parse::<u32>().unwrap() - 1;
            x.rm_edge(node1, node2);
        }
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else {
        json!({ "status": "FAIL"}).to_string()
    }
}

#[post("/node/add/<method>", data = "<var>")]
fn add_node(method: String, var: String) -> String {
    let start = Instant::now();
    let json: GraphJson = serde_json::from_str(&var[..]).unwrap();
    if method == "AdjacencyList" {
        let mut x = AdjacencyList::from_json(var);
        for _r in json.par_vertices.iter() {
            //let node = r.parse::<u32>().unwrap() - 1;
            x.add_node();
        } 
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else if method == "AdjacencyMatrix"{
        let mut x = AdjacencyMatrix::from_json(var);
        for _r in json.par_vertices.iter() {
            //let node = r.parse::<u32>().unwrap() - 1;
            x.add_node();
        }
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else {
        json!({ "status": "FAIL"}).to_string()
    }
}

#[post("/node/remove/<method>", data = "<var>")]
fn rm_node(method: String, var: String) -> String {
    let start = Instant::now();
    let json: GraphJson = serde_json::from_str(&var[..]).unwrap();
    if method == "AdjacencyList" {
        let mut x = AdjacencyList::from_json(var);
        for r in json.par_vertices.iter() {
            let node = r.parse::<u32>().unwrap() - 1;
            x.rm_node(node);
        } 
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else if method == "AdjacencyMatrix"{
        let mut x = AdjacencyMatrix::from_json(var);
        for r in json.par_vertices.iter() {
            let node = r.parse::<u32>().unwrap() - 1;
            x.rm_node(node);
        }
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": x.get_GraphJson() }).to_string()
    } else {
        json!({ "status": "FAIL"}).to_string()
    }
}

#[post("/node/neighborhood/<method>/<node>", data = "<var>")]
fn neighborhood(method: String, node: u32, var: String) -> String {
    let start = Instant::now();
    if method == "AdjacencyList" {
        let x = AdjacencyList::from_json(var);
        let nh : Vec<u32> = x.get_neighborhood(node-1).iter().map(|x| x+1).collect();
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": nh }).to_string()
    } else if method == "AdjacencyMatrix"{
        let x = AdjacencyMatrix::from_json(var);
        let nh : Vec<u32> = x.get_neighborhood(node-1).iter().map(|x| x+1).collect();
        json!({ "status": "OK", "duration": start.elapsed().as_secs_f64(), "data": nh }).to_string()
    } else {
        json!({ "status": "FAIL"}).to_string()
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
    for r in 0..edges {
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

// FRONT-END

#[get("/public/src/<file..>")]
fn file(file : PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/src/").join(file)).ok()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/src/index.html")
}

fn main() {
    let cors = rocket_cors::Cors::default();
    
    rocket::ignite()
        .mount("/", routes![index, file])
        .mount("/api", routes![read])
        .mount("/api/exec", routes![add_edge, rm_edge, add_node, rm_node, neighborhood, generate_random_graphs])
        .attach(cors)
    .launch();
}