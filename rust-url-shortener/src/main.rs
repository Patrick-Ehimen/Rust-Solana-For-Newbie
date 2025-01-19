use actix_web::{web, App, HttpServer};
use crate::routes::init_routes;
use env_logger;

mod routes;
mod handlers;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}