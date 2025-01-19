// pub mod url_shortener;

// use actix_web::{web, HttpResponse, Responder};

// pub fn configure_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(shorten_url)
//        .service(get_original_url);
// }

// async fn shorten_url() -> impl Responder {
//     HttpResponse::Ok().body("Shorten URL endpoint")
// }

// async fn get_original_url() -> impl Responder {
//     HttpResponse::Ok().body("Get original URL endpoint")
// }
use actix_web::web;

pub mod url_shortener;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(url_shortener::shorten_url)
        .service(url_shortener::get_original_url);
}
