mod health_check;

use actix_web::web::{self, ServiceConfig};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{shares::errors::server_error::ServerError, types::ServerResponse};

#[derive(OpenApi)]
#[openapi(
    paths(health_check::hello),
    components(schemas(), responses()),
    modifiers(),
    security(),
    tags()
)]
pub struct ApiDoc;

async fn default_service() -> ServerResponse {
    Err(ServerError::NotFound {
        message: "Service not found".into(),
    })
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/api/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
    cfg.service(
        web::scope("/api")
            // INFO: Health check service
            .service(health_check::hello),
    );
    cfg.default_service(web::route().to(default_service));
}
