use regex::Regex;
use serde_json::Value;

use super::ModuleDBType;
use crate::r#type::{ServiceError, ServerError};

pub trait ModuleDBEscape<T> {
    fn try_escape_to_string(&self, item: T) -> Result<String, ServiceError>;
}

impl ModuleDBEscape<()> for String {
    fn try_escape_to_string(&self, _: ()) -> Result<String, ServiceError> {
        if Regex::new(r"^[a-zA-Z0-9_]+$").unwrap().is_match(&self) {
            Ok(self.to_string())
        } else {
            Err(ServerError::DBAccessError.into())
        }
    }
}

impl ModuleDBEscape<Value> for ModuleDBType {
    fn try_escape_to_string(&self, item: Value) -> Result<String, ServiceError> {
        match self {
            Self::ARRAY(i) => {
                if let Value::Array(arr) = item {
                    let mut v = Vec::new();
                    for a in arr {
                        v.push(i.try_escape_to_string(a)?);
                    }
                    Ok(escape(&v.join(","), "{", "}"))
                } else {
                    Err(ServerError::DBAccessError.into())
                }
            },
            Self::NULLABLE(i) => {
                if Value::Null == item {
                    Ok("Null".to_string())
                } else {
                    i.try_escape_to_string(item)
                }
            },
            Self::INT | Self::BIG_INT |
            Self::SERIAL | Self::BIG_SERIAL => {
                if let Value::Number(v) = item {
                    if v.is_i64() {
                        Ok(v.to_string())
                    } else {
                        Err(ServerError::DBAccessError.into())
                    }
                } else {
                    Err(ServerError::DBAccessError.into())
                }
            },
            Self::TEXT => {
                if let Value::String(v) = item {
                    Ok(escape(&v, "", ""))
                } else {
                    Err(ServerError::DBAccessError.into())
                }
            },
            Self::OBJECT => {
                Ok(escape(&item.to_string(), "", ""))
            }
        }
    }
}

fn escape(item: &str, prefix: &str, suffix: &str) -> String {
    format!("'{}{}{}'", prefix, item.replace("'", "''"), suffix).to_string()
}