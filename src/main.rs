mod configs;
mod constants;
mod modules;
mod server;
mod shares;
mod types;

#[actix_web::main]
async fn main() {
    server::start().await;
}
