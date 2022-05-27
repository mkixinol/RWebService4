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
use actix_session::{Session};

use crate::r#type::{UDbId, ServiceError};

pub struct RWAuth (Vec<UDbId>);

impl RWAuth {
    pub fn new(auth: Vec<UDbId>) -> Self {
        Self(auth)
    }

    pub fn check_session(&self, session: &Session) -> Result<(), ServiceError>{
        Ok(())
    }

    pub fn check_token(&self) -> Result<(), ServiceError>{
        Ok(())
    }
}