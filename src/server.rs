use actix_web::{middleware::Logger, App, HttpServer};

use crate::{modules::config, shares::middlewares::logger::setup_logger};

pub async fn start() {
    setup_logger();

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(config))
        .bind(("127.0.0.1", 3000))
        .unwrap_or_else(|err| panic!("Cannot bind socket address: {}", err))
        .run()
        .await
        .unwrap_or_else(|err| panic!("{}", err));
}
