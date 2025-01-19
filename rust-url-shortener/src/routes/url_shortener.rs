use actix_web::{post, get, web, HttpResponse, Responder};
use crate::handlers::url_handler::{create_short_url, redirect_to_original_url};

#[post("/shorten")]
async fn shorten_url(url: web::Json<String>) -> impl Responder {
    let short_url = create_short_url(&url);
    HttpResponse::Ok().json(short_url)
}

#[get("/{short_url}")]
async fn get_original_url(path: web::Path<String>) -> impl Responder {
    let short_url = path.into_inner();
    match redirect_to_original_url(&short_url) {
        Some(original_url) => HttpResponse::Found().append_header(("Location", original_url)).finish(),
        None => HttpResponse::NotFound().finish(),
    }
}