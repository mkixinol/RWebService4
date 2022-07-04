use std::{
    fmt::Debug,
    str::FromStr,
    collections::HashMap,
    task::{Context, Poll},
    pin::Pin,
    future::Future,
};
use serde::{
    Serialize,
    Deserialize,
    de::DeserializeOwned
};
use serde_json::Value;
use futures_core::{ready, Stream};
use futures_core::TryStream;
use futures_util::TryStreamExt;
use once_cell::sync::OnceCell;
use actix_web::{web, dev, http, HttpRequest, FromRequest, Error};
use actix_utils::future::Ready;
use actix_multipart::Multipart;

use crate::r#type::{ServiceError};

static TEMP_DIR: OnceCell<String> = OnceCell::new();

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestValue<P, Q, B> {
    path: P,
    query: Q,
    body: B,
}

impl<P, Q, B> RequestValue<P, Q, B> {
    pub fn set_config(dir: &str) {
        let _ = TEMP_DIR.set(dir.to_string());
    }
}

impl<P, Q, B> FromRequest for RequestValue<P, Q, B>
where
    P: DeserializeOwned + Debug,
    Q: DeserializeOwned + Debug,
    B: DeserializeOwned + Debug,
{
    type Error = ServiceError;
    type Future = RequestValueFut<P, Q, B>;

    #[inline]
    fn from_request(request: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        RequestValueFut::new(request, payload)
    }
}

pub enum RequestValueInner<T> {
    Marker,
    Json(web::JsonBody<T>),
    Multipart(Ready<Result<Multipart, Error>>),
    Urlencoded(web::JsonBody<T>),
}

pub struct RequestValueFut<P, Q, B> {
    polled: bool,
    request: HttpRequest,
    path: RequestValueInner<P>,
    query: RequestValueInner<Q>,
    body: RequestValueInner<B>
}
impl<P, Q, B> RequestValueFut<P, Q, B>
where
    P: DeserializeOwned + Debug,
    Q: DeserializeOwned + Debug,
    B: DeserializeOwned + Debug,
{
    pub fn new(request: &HttpRequest, payload: &mut dev::Payload) -> Self {
        let content_type = match &request.headers().get(http::header::CONTENT_TYPE) {
            Some(t) => t.to_str().unwrap_or(""),
            None => "",
        };
        let content_type = mime::Mime::from_str(content_type).unwrap_or(mime::STAR_STAR);
        let prefix = content_type.type_();
        let subtype = content_type.subtype();
        let body = match prefix {
            mime::APPLICATION => {
                match subtype {
                    mime::JSON => {
                        RequestValueInner::Json(
                            web::JsonBody::new(request, payload, None, true).limit(2097152)
                        )
                    }
                    mime::WWW_FORM_URLENCODED => {
                        todo!()
                    }
                    _ => {
                        RequestValueInner::Marker
                    }
                }
            }
            mime::MULTIPART => {
                match subtype {
                    mime::FORM_DATA => {
                        RequestValueInner::Multipart(
                            Multipart::from_request(request, payload)
                        )
                    }
                    _ => {
                        RequestValueInner::Marker
                    }
                }
            }
            _ => {RequestValueInner::Marker}
        };
        
        Self {
            polled: false,
            request: request.clone(),
            path: RequestValueInner::Marker,
            query: RequestValueInner::Marker,
            body: body
        }
    }
}
impl<P, Q, B> Future for RequestValueFut<P, Q, B>
where
    P: DeserializeOwned + Debug,
    Q: DeserializeOwned + Debug,
    B: DeserializeOwned + Debug,
{
    type Output = Result<RequestValue<P, Q, B>, ServiceError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut body_fn = || {
            match &mut this.body {
                RequestValueInner::Json(json) => {
                    match Pin::new(json).poll(cx) {
                        Poll::Pending => None,
                        Poll::Ready(r) => Some(r.ok())
                    }
                }
                RequestValueInner::Multipart(multipart) => {
                    println!(".");
                    match Pin::new(multipart).poll(cx) {
                        Poll::Pending => None,
                        Poll::Ready(ready) => {
                            if let Ok(mut form) = ready {
                                let f = || async move {
                                    println!("._");
                                    while let Ok(Some(mut field)) = form.try_next().await {
                                        let filename = field.content_disposition().get_filename();
                                        let filename2 = field.content_disposition().get_name();
                                        println!("{:?},{:?}", filename, filename2);
                                    }
                                };
                                let _: () = Box::pin(&mut f()).poll(cx);
                                /*
                                println!("...");
                                while let mut field = Pin::new(&mut form).poll_next(cx) {
                                    println!("_");
                                }
                                */
                            }
                            Some(serde_json::from_value(Value::Null).ok())
                        }
                    }
                }
                _ => {
                    println!("form unexpected");
                    Some(serde_json::from_value(Value::Null).ok())
                }
            }
        };
        if !this.polled {
            this.polled = true;
            let body = body_fn();
            if let Some(body) = body {
                let path: HashMap<&str, &str> = this.request.match_info().iter().collect();
                let pathres = serde_json::from_value(serde_json::to_value(path).unwrap());
                let mut query = web::Query::extract(&this.request);
                let queryfut: Result<web::Query<Q>, _> = ready!(Pin::new(&mut query).poll(cx));

                let res = if body.is_some() && pathres.is_ok() && queryfut.is_ok() {
                    Ok(
                        RequestValue {
                            path: pathres.unwrap(),
                            query: queryfut.unwrap().into_inner(),
                            body: body.unwrap()
                        }
                    )
                } else {
                    Err(ServiceError::HTTPRequestBadRequest)
                };
                println!("{:#?}", res);
                Poll::Ready(res)
            } else {
                this.polled = false;
                Poll::Pending
            }
        } else {
            Poll::Pending
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subval {
    id: String
}