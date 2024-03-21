use actix_web::{get, HttpResponse};

use crate::types::ServerResponse;

#[utoipa::path(
    get,
    path="/api/",
    responses(
        (status=200, description="Server health check")
    )
)]
#[get("/")]
async fn hello() -> ServerResponse {
    Ok(HttpResponse::Ok().body("Hello World"))
}
