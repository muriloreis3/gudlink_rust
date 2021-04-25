// mod link;
use actix_web::{web, HttpRequest, HttpResponse, Result};

#[get("/")]
pub async fn index() -> HttpResponse {
    let data = json!({
        "name": "Hello, World!"
    });
    HttpResponse::Ok().json(&data)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}