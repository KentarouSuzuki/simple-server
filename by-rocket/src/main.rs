#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;
extern crate rocket_contrib;
extern crate server_info;

mod route;
mod service;
mod view;

fn main() {
    rocket::ignite().mount("/", routes![route::root]).launch();
}
