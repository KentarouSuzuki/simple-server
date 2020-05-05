#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod route;
mod service;

fn main() {
    rocket::ignite().mount("/", routes![route::root]).launch();
}
