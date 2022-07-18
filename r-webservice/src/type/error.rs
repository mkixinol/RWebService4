use std::convert::From;
use serde::{Serialize, Deserialize};

use actix_web::{error::ResponseError, HttpResponse};
use actix_web::http::{header::ContentType, StatusCode};

use derive_more::{Display, Error};

#[derive(Debug, Display, Error, Serialize, Deserialize)]
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
    #[display(fmt = "Payload Too Large")]
    HTTPRequestPayloadTooLarge,
    #[display(fmt = "Internal Server Error")]
    HTTPRequestInternalServerError(ServerError),
}

#[derive(Debug, Display, Error, Serialize, Deserialize)]
pub enum ServerError {
    #[display(fmt = "Database Access Error")]
    DBAccessError,
    #[display(fmt = "Session Access Error")]
    SessionAccessError,
    #[display(fmt = "Template Routing Error")]
    TemplateRoutingError,
    #[display(fmt = "Template Module Error")]
    TemplateModuleError,
    #[display(fmt = "Template Type Error")]
    TemplateTypeError,
    #[display(fmt = "Serde Parse Error")]
    SerdeParseError,
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
            Self::HTTPRequestPayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::HTTPRequestInternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ServerError> for ServiceError {
    fn from(item: ServerError) -> Self {
        ServiceError::HTTPRequestInternalServerError(item)
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(item: serde_json::Error) -> Self {
        ServerError::SerdeParseError.into()
    }
}