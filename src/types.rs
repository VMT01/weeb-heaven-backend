use actix_web::HttpResponse;

use crate::shares::errors::server_error::ServerError;

pub type ServerResponse = Result<HttpResponse, ServerError>;
