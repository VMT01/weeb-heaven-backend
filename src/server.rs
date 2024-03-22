use actix_web::{middleware::Logger, App, HttpServer};

use crate::{configs::ENV_VALUES, modules::config, shares::middlewares::logger::setup_logger};

pub async fn start() {
    dotenv::dotenv().unwrap_or_else(|err| panic!("Cannot setup env: {}", err));
    setup_logger();

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(config))
        .bind(("127.0.0.1", ENV_VALUES.port))
        .unwrap_or_else(|err| panic!("Cannot bind socket address: {}", err))
        .run()
        .await
        .unwrap_or_else(|err| panic!("{}", err));
}
