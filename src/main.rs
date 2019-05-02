
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::io;
use rocket::response::{NamedFile};
use serde_json::{Result, Value, json};
use rust_graph_theory::graph_theory::adjacency_list;
use rust_graph_theory::graph_theory::adjacency_matrix;

#[get("/submit", data = "<var>")]
fn submit(var: String) {
    let v: Value = serde_json::from_str(&var[..]).unwrap();
    print!("{:?}", v["nome"]);
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/json")]
fn json() -> String {
    json!({ "hi": "world" }).to_string()
}

#[get("/teste")]
fn teste2() -> String {
    let x = adjacency_list::AdjacencyList::new(10,10);
    json!({ "hi": 5, "teste": x }).to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![json,teste2, submit])
    .launch();
}