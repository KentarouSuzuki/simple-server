use actix_web::{App, HttpServer};

mod route;
mod service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().service(route::root()) })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
