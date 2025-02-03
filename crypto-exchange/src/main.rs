// src/main.rs

use actix_web::{web, App, HttpServer, Responder};
use exchange::Exchange;

mod exchange;
mod models;
mod utils;

async fn health_check() -> impl Responder {
    "Crypto Exchange is running!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let exchange = Exchange::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(exchange.clone()))
            .route("/health", web::get().to(health_check))
            // Additional routes for trading, order management, etc. can be added here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}