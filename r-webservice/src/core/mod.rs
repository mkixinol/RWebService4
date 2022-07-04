use std::collections::HashMap;
use serde_json::Value;

pub mod index;
pub mod tls;
pub mod cache;
pub mod server;
pub mod request;
pub mod database;

pub type RequestValueAny = request::RequestValue<HashMap<String, String>, HashMap<String, String>, Value>;