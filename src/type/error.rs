use actix_web::{error::ResponseError, HttpResponse};
use actix_web::http::{header::ContentType, StatusCode};

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    // IOError(std::io::Error),
    // TLSError(rustls::TLSError),
    #[display(fmt = "Bad Request")]
    HTTPRequestBadRequest,
    #[display(fmt = "Unauthorized")]
    HTTPRequestUnauthorized,
    #[display(fmt = "Forbidden")]
    HTTPRequestForbidden,
    #[display(fmt = "Not Found")]
    HTTPRequestNotFound,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body("")
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::HTTPRequestBadRequest => StatusCode::BAD_REQUEST,
            Self::HTTPRequestUnauthorized => StatusCode::UNAUTHORIZED,
            Self::HTTPRequestForbidden => StatusCode::FORBIDDEN,
            Self::HTTPRequestNotFound => StatusCode::NOT_FOUND,
        }
    }
}
