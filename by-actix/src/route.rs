use actix_web::{web, Resource};
use crate::service::hello;

pub fn root() -> Resource {
    web::resource("/").route(web::to(hello))
}