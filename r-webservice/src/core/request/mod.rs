mod multipart;

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
use futures_core::{ready, Stream, TryStream};
use futures_util::TryStreamExt;
use once_cell::sync::OnceCell;
use actix_web::{web, dev, http, HttpRequest, FromRequest, Error};
use actix_utils::future::Ready;
use actix_multipart::Multipart;

use crate::r#type::{ServiceError};

static TEMP_DIR: OnceCell<RequestConfig> = OnceCell::new();

pub struct RequestConfig {
    dir: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestValue<P, Q, B> {
    path: P,
    query: Q,
    body: B,
}

impl<P, Q, B> RequestValue<P, Q, B> {
    pub fn set_config(dir: &str) {
        let _ = TEMP_DIR.set(
            RequestConfig{
                dir: dir.to_string()
            }
        );
    }
}

impl<P, Q, B> FromRequest for RequestValue<P, Q, B>
where
    P: DeserializeOwned + Debug + Unpin,
    Q: DeserializeOwned + Debug + Unpin,
    B: DeserializeOwned + Debug + Unpin,
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
    MarkerPending,
    MarkerProcessed,
    MarkerDone(T),
    Query(Ready<Result<web::Query<T>, Error>>),
    Json(web::JsonBody<T>),
    Multipart(Ready<Result<Multipart, Error>>),
    Urlencoded(web::JsonBody<T>),
}

pub struct RequestValueFut<P, Q, B> {
    request: HttpRequest,
    path: RequestValueInner<P>,
    query: RequestValueInner<Q>,
    body: RequestValueInner<B>,
}
impl<P, Q, B> RequestValueFut<P, Q, B>
where
    P: DeserializeOwned + Debug + Unpin,
    Q: DeserializeOwned + Debug + Unpin,
    B: DeserializeOwned + Debug + Unpin,
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
            request: request.clone(),
            path: RequestValueInner::Marker,
            query: RequestValueInner::Marker,
            body: body
        }
    }
}
impl<P, Q, B> Future for RequestValueFut<P, Q, B>
where
    P: DeserializeOwned + Debug + Unpin,
    Q: DeserializeOwned + Debug + Unpin,
    B: DeserializeOwned + Debug + Unpin
{
    type Output = Result<RequestValue<P, Q, B>, ServiceError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // Path
        if let RequestValueInner::Marker = this.path {
            let path: HashMap<&str, &str> = this.request.match_info().iter().collect();
            let pathres = serde_json::from_value(serde_json::to_value(path)?);
            this.path = RequestValueInner::MarkerDone(pathres?);
        }

        // Query
        if let RequestValueInner::Marker = &this.query {
            let query = web::Query::extract(&this.request);
            this.query = RequestValueInner::Query(query);
        }
        if let RequestValueInner::Query(query) = &mut this.query {
            if let Poll::Ready(res) = Pin::new(query).poll(cx) {
                this.query = RequestValueInner::MarkerDone(res.unwrap().into_inner());
            } else {
                return Poll::Pending;
            }
        }

        // Body
        let mut f = |body: &mut RequestValueInner<B>| {
            if let RequestValueInner::MarkerPending = body {
                Ok(None)
            } else if let RequestValueInner::Json(json) = body {
                if let Poll::Ready(ready) = Pin::new(json).poll(cx) {
                    Ok(Some(RequestValueInner::MarkerDone(ready.unwrap())))
                } else {
                    Ok(None)
                }
            } else if let RequestValueInner::Multipart(multipart) = body {
                if let Poll::Ready(payload) = Pin::new(multipart).poll(cx) {
                    if let Ok(mut data) = payload {
                        while let form = Pin::new(&mut data).try_poll_next(cx) {

                        }
                        Ok(None)
                    } else {
                        Err(ServiceError::HTTPRequestBadRequest)
                    }
                } else {
                    Ok(None)
                }
                /*
                if let actix_utils::future::Ready(ready) = multipart {

                }
                let mut pin = Pin::new(multipart);
                while let polled = pin.poll_next(cx) {

                }
                */
                //Err(ServiceError::HTTPRequestBadRequest)
            } else {
                Err(ServiceError::HTTPRequestBadRequest)
            }
        };

        // Return val
        let mut body = std::mem::replace(&mut this.body, RequestValueInner::MarkerPending);
        match f(&mut body) {
            Ok(Some(result)) => {
                this.body = result;
                if let RequestValueInner::MarkerDone(path) = std::mem::replace(&mut this.path, RequestValueInner::MarkerProcessed) {
                    if let RequestValueInner::MarkerDone(query) = std::mem::replace(&mut this.query, RequestValueInner::MarkerProcessed) {
                        if let RequestValueInner::MarkerDone(body) = std::mem::replace(&mut this.body, RequestValueInner::MarkerProcessed) {
                            cx.waker().wake_by_ref();
                            Poll::Ready(
                                Ok(
                                    RequestValue {
                                        path: path,
                                        query: query,
                                        body: body
                                    }
                                )
                            )
                        } else { unreachable!() }
                    } else { unreachable!() }
                } else { unreachable!() }
            }
            Ok(None) => {
                this.body = body;
                Poll::Pending
            }
            Err(e) => {
                Poll::Ready(Err(e))
            }
        }
        /*
        match &mut this.body {
            RequestValueInner::() => {
                body_poll!(this.body, multipart, cx, r, serde_json::from_value(Value::Null).unwrap());
                /*
                match Pin::new(multipart).poll(cx) {
                    Poll::Pending => None,
                    Poll::Ready(ready) => {
                        if let Ok(mut form) = ready {
                            println!("._");
                            while let Ok(Some(mut field)) = form.try_next().await {
                                let filename = field.content_disposition().get_filename();
                                let filename2 = field.content_disposition().get_name();
                                println!("{:?},{:?}", filename, filename2);
                            }
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
                        */
            }
            _ => {
                println!("form unexpected");
                /*
                Some(serde_json::from_value(Value::Null).ok())
                */
            }
        };
        */
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subval {
    id: String
}