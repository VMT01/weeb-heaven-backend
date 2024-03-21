mod modules;
mod shares;
mod types;

use actix_web::{App, HttpServer};

use crate::modules::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
