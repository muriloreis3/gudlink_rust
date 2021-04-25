// mod link;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use handlebars::Handlebars;

#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "name": "Hello, World!"
    });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(&body)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}