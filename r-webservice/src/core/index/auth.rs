
use actix_session::{Session};

use crate::r#type::{IDbNo, UDbId, ServiceError};

pub enum RWAuth {
    Server(RWAuthServer),
    Client(RWJWTClient)
}

impl RWAuth {
    pub const AUTH_TYPE_NONE:   IDbNo = 0;
    pub const AUTH_TYPE_GROUP:  IDbNo = 1;
    pub const AUTH_TYPE_MEMBER: IDbNo = 2;
    pub const AUTH_TYPE_CUSTOM: IDbNo = 3;

    pub fn new(_auth: Vec<UDbId>) -> Self {
        Self::Client(RWJWTClient{_head: "".to_string()})
    }

    pub fn check_session(&self, _session: &Session) -> Result<Vec<UDbId>, ServiceError>{
        Ok(vec![])
    }

    pub fn check_token(&self) -> Result<Vec<UDbId>, ServiceError>{
        Ok(vec![])
    }
}

pub struct RWAuthServer {
    _mode: IDbNo,
    _item: Vec<UDbId>
}

pub struct RWJWTClient {
    _head: String
}

/*
use actix_redis::RedisSession;

pub struct RWSession {
    host: String,
    port: String,
    name: String
}

impl RWSession {
    pub fn session(&self) -> RedisSession {
        RedisSession::new(
            format!("{}:{}", &self.host, &self.port),
            &[0u8; 32]
        )
        .cookie_name(&self.name)
    }
}
*/