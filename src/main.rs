#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod models;
mod routes;

// WebAPIのURLルーティングはroutes.rsに移動する
use routes::*;

fn main() {
    rocket::ignite().mount("/", routes![languages]).launch();
}
