pub mod libs;
mod tests;

#[macro_use] extern crate actix_web;
#[macro_use] extern crate serde_json;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use crate::libs::hbs_helpers::to_lower_helper;

fn handlebars_config() -> web::Data<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./templates")
        .unwrap();
    handlebars.register_helper("toLower", Box::new(to_lower_helper));
    web::Data::new(handlebars)
}

pub fn static_files() -> fs::Files {
    fs::Files::new("/public", "./static")
}

pub async fn run(address: &str) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(libs::error::error_handlers())
            .app_data(handlebars_config())
            .configure(libs::routes::config)
            .service(static_files())
    })
    .bind(address)?
    .run()
    .await
}
