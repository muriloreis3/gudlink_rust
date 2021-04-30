// mod link;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use handlebars::Handlebars;
use super::model::page::Page;
use crate::tests::test_dummy_data_insertion;

#[get("/api/test")]
pub async fn test() -> HttpResponse {
    println!("Inserting data in db");
    test_dummy_data_insertion().await.unwrap();
    HttpResponse::Ok().body(json!({}))
}

#[get("/{handle}")]
pub async fn get_page(hb: web::Data<Handlebars<'_>>,handle: web::Path<String>) -> HttpResponse {
    let page = match Page::find_by_handle(handle.to_string()).await {
        Ok(p_list) => p_list,
        Err(_) => {
            let body = hb.render("unexpected", &json!({})).unwrap();
            return HttpResponse::Ok().body(&body);
        }
    }; 
    let body = hb.render("gud-page", &page).unwrap();
    HttpResponse::Ok().body(&body)
}

#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "name": "Hello, World!"
    });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(&body)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(get_page).service(test);
}