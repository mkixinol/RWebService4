use std::collections::HashMap;
use serde_json::Value;
use sqlx::PgPool;
use actix_web::HttpRequest;
use actix_session::Session;

use crate::r#type::{IDbNo, UDbId, ServiceError, ServerError};
use crate::core::cache::RWCache;
use crate::module::*;
use super::EngineValue;

pub struct EngineRequest();
impl EngineRequest
{
    //pub const ENGINE_REQUEST_PATH_ROOT:     &'static str = "";
    //pub const ENGINE_REQUEST_PATH_DELIM:    &'static str = ".";
    //pub const ENGINE_REQUEST_PATH_HEADER:   &'static str = "header";
    //pub const ENGINE_REQUEST_PATH_PARAM:    &'static str = "param";
    //pub const ENGINE_REQUEST_PATH_SESSION:  &'static str = "session";
    //pub const ENGINE_REQUEST_PATH_COOKIE:   &'static str = "cookie";
    //pub const ENGINE_REQUEST_PATH_BODY:     &'static str = "body";

    pub fn new() -> Self {
        Self()
    }
    pub async fn execute<'a>(
        &self,
        _pool: &'a PgPool,
        _cache: &'a RWCache,
        _option: &Option<HashMap<IDbNo, UDbId>>,
        template: &Template,
        _request: &HttpRequest,
        _session: &Session,
        //body: &RequestBody
    ) -> Result<(bool, EngineValue), ServiceError> {
        let value = Value::Null;
        /*
        if let Some(root_key) = template.option.get(Self::ENGINE_REQUEST_PATH_ROOT) {
            value = Self::parse(&root_key, param, request, session, body)?;
        } else {
            for (k, v) in template.option.iter() {
                let mut value_inner = Map::new();
                value_inner.insert(
                    k.to_string(),
                    Self::parse(v, param, request, session, body)?
                );
                value = Value::Object(value_inner);
            }
        }
*/
        match template.path.as_str() {
            EngineValue::ENGINE_TYPE_ANY => {
                Ok((true, EngineValue::Any(value)))
            },
            EngineValue::ENGINE_TYPE_SEARCHDETAIL => {
                match serde_json::from_value(value) {
                    Ok(s) => {
                        Ok((true, EngineValue::SearchDetail(s)))
                    },
                    Err(_) => {
                        Ok((false, EngineValue::None))
                    }
                }
            },
            EngineValue::ENGINE_TYPE_SEARCHLIST => {
                match serde_json::from_value(value) {
                    Ok(s) => {
                        Ok((true, EngineValue::SearchList(s)))
                    },
                    Err(_) => {
                        Ok((false, EngineValue::None))
                    }
                }
            },
            EngineValue::ENGINE_TYPE_UPSERT => {
                Ok((true, EngineValue::Upsert(value)))
            },
            EngineValue::ENGINE_TYPE_MODULE => {
                match serde_json::from_value(value) {
                    Ok(s) => {
                        Ok((true, EngineValue::Module(s)))
                    },
                    Err(_) => {
                        Ok((false, EngineValue::None))
                    }
                }
            },
            EngineValue::ENGINE_TYPE_MODULEARRAY => {
                match serde_json::from_value(value) {
                    Ok(s) => {
                        Ok((true, EngineValue::ModuleArray(s)))
                    },
                    Err(_) => {
                        Ok((false, EngineValue::None))
                    }
                }
            },
            EngineValue::ENGINE_TYPE_SERVICEERROR => {
                match serde_json::from_value(value) {
                    Ok(s) => {
                        Ok((true, EngineValue::ServiceError(s)))
                    },
                    Err(_) => {
                        Ok((false, EngineValue::None))
                    }
                }
            },
            EngineValue::ENGINE_TYPE_SERVERERROR => {
                match serde_json::from_value(value) {
                    Ok(s) => {
                        Ok((true, EngineValue::ServerError(s)))
                    },
                    Err(_) => {
                        Ok((false, EngineValue::None))
                    }
                }
            },
            EngineValue::ENGINE_TYPE_NONE => {
                Ok((true, EngineValue::None))
            },
            _ => {
                Err(ServerError::TemplateModuleError.into())
            }
        }
    }
    /*
    pub fn parse<'a>(
        key: &str,
        param: &HashMap<String, String>,
        request: &HttpRequest,
        session: &Session,
        body: &RequestBody
    ) -> Result<Value, ServiceError> {
        let mut value = Value::Null;
        let mut keys_iter = key.split(Self::ENGINE_REQUEST_PATH_DELIM);
        if let Some(k0) = keys_iter.next() {
            let k1 = keys_iter.next().unwrap_or("");
            match k0 {
                Self::ENGINE_REQUEST_PATH_HEADER => {
                    value = Value::String(
                        request.headers().get(k1).unwrap_or(&HeaderValue::from_static("")).to_str().unwrap_or("").to_string()
                    );
                },
                Self::ENGINE_REQUEST_PATH_PARAM => {
                    match k1 {
                        "" => {
                            value = Value::Object(
                                Map::from_iter(
                                    param.iter().map(
                                        |(k, v)| (k.to_string(), Value::String(v.to_string()))
                                    )
                                )
                            );
                        },
                        s => {
                            value = Value::String(s.to_string());
                        }
                    }
                },
                Self::ENGINE_REQUEST_PATH_SESSION => {
                    if let Ok(v) = session.get(k1) {
                        value = v.unwrap_or(Value::Null);
                    } else {
                        return Err(ServerError::SessionAccessError.into());
                    }
                },
                Self::ENGINE_REQUEST_PATH_COOKIE => {

                },
                Self::ENGINE_REQUEST_PATH_BODY => {

                },
                _ => {}
            }
        }
        Ok(value)
    }
    */
}
