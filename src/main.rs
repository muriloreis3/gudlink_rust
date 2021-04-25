use gudlink::libs::constants;
use std::{env, io};

fn get_address() -> String {
    let host = env::var("HOST")
        .ok()
        .unwrap_or_else(|| constants::HOST.to_string());
    let port = env::var("PORT")
        .ok()
        .unwrap_or_else(|| constants::PORT.to_string());
    format!("{}:{}", host, port)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    gudlink::run(&get_address()).await
}
