use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "Internal Server Error: {}", message)]
    InternalError { message: String },

    #[display(fmt = "Not found: {}", message)]
    NotFound { message: String },
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ServerError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::NotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}
