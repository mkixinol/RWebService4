
use actix_web::{web, dev, http, HttpRequest, FromRequest, Error};
use actix_utils::future::Ready;
use actix_multipart::Multipart;

use super::RequestConfig;

struct MultipartFut {
    config: RequestConfig,
    fut: Ready<Result<Multipart, Error>>
}

