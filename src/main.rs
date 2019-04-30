
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::io;
use rocket::response::{NamedFile};
use rocket_contrib::json::{Json, JsonValue};
use rust_graph_theory::graph_theory::adjacency_list;
use rust_graph_theory::graph_theory::adjacency_matrix;


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/json")]
fn json() -> Json<JsonValue> {
    Json(json!({ "hi": "world" }))
}

#[get("/teste")]
fn teste2() -> Json<JsonValue> {
    Json(json!({ "hi": 5 }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![json,teste2])
    .launch();
}