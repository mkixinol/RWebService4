use std::convert::From;
use actix_web::http::Method as HttpMethod;
use crate::r#type::{UDbFlg, UDbFlgBit};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Method(pub UDbFlgBit);

impl Method {
    pub const METHOD_TYPE_OPTIONS:  UDbFlgBit = 1 << 0;
    pub const METHOD_TYPE_GET:      UDbFlgBit = 1 << 1;
    pub const METHOD_TYPE_POST:     UDbFlgBit = 1 << 2;
    pub const METHOD_TYPE_PUT:      UDbFlgBit = 1 << 3;
    pub const METHOD_TYPE_DELETE:   UDbFlgBit = 1 << 4;
    pub const METHOD_TYPE_HEAD:     UDbFlgBit = 1 << 5;
    pub const METHOD_TYPE_TRACE:    UDbFlgBit = 1 << 6;
    pub const METHOD_TYPE_CONNECT:  UDbFlgBit = 1 << 7;
    pub const METHOD_TYPE_PATCH:    UDbFlgBit = 1 << 8;

    pub const OPTIONS:  Method = Method(Self::METHOD_TYPE_OPTIONS);
    pub const GET:      Method = Method(Self::METHOD_TYPE_GET);
    pub const POST:     Method = Method(Self::METHOD_TYPE_POST);
    pub const PUT:      Method = Method(Self::METHOD_TYPE_PUT);
    pub const DELETE:   Method = Method(Self::METHOD_TYPE_DELETE);
    pub const HEAD:     Method = Method(Self::METHOD_TYPE_HEAD);
    pub const TRACE:    Method = Method(Self::METHOD_TYPE_TRACE);
    pub const CONNECT:  Method = Method(Self::METHOD_TYPE_CONNECT);
    pub const PATCH:    Method = Method(Self::METHOD_TYPE_PATCH);

    pub const FLAGS: [UDbFlgBit; 9] = [
        Self::METHOD_TYPE_OPTIONS,
        Self::METHOD_TYPE_GET,
        Self::METHOD_TYPE_POST,
        Self::METHOD_TYPE_PUT,
        Self::METHOD_TYPE_DELETE,
        Self::METHOD_TYPE_HEAD,
        Self::METHOD_TYPE_TRACE,
        Self::METHOD_TYPE_CONNECT,
        Self::METHOD_TYPE_PATCH,
    ];

    pub fn from_flag(flag: UDbFlg) -> Vec<Self> {
        let mut methods = vec![];
        for m in Self::FLAGS {
            if (flag & m) > 0 {
                methods.push(Method(m));
            }
        }
        methods
    }
}

impl From<Method> for HttpMethod {
    fn from(item: Method) -> Self {
        match item {
            Method::OPTIONS => Self::OPTIONS,
            Method::GET     => Self::GET,
            Method::POST    => Self::POST,
            Method::PUT     => Self::PUT,
            Method::DELETE  => Self::DELETE,
            Method::HEAD    => Self::HEAD,
            Method::TRACE   => Self::TRACE,
            Method::CONNECT => Self::CONNECT,
            Method::PATCH   => Self::PATCH,
            _               => Self::GET,
        }
    }
}

impl From<&HttpMethod> for Method {
    fn from(item: &HttpMethod) -> Self {
        match item {
            &HttpMethod::OPTIONS => Self::OPTIONS,
            &HttpMethod::GET     => Self::GET,
            &HttpMethod::POST    => Self::POST,
            &HttpMethod::PUT     => Self::PUT,
            &HttpMethod::DELETE  => Self::DELETE,
            &HttpMethod::HEAD    => Self::HEAD,
            &HttpMethod::TRACE   => Self::TRACE,
            &HttpMethod::CONNECT => Self::CONNECT,
            &HttpMethod::PATCH   => Self::PATCH,
            _                   => Self::GET
        }
    }
}