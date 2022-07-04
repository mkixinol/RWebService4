use serde::{Deserialize, Serialize};

use crate::r#type::{UDbId, IDbNo};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auth {
    mode: IDbNo,
    item: Vec<UDbId>
}

impl Auth {
    pub const AUTH_TYPE_NONE:   IDbNo = 0;
    pub const AUTH_TYPE_GROUP:  IDbNo = 1;
    pub const AUTH_TYPE_MEMBER: IDbNo = 2;
    pub const AUTH_TYPE_CUSTOM: IDbNo = 3;

    pub fn new(mode: IDbNo, item: Vec<UDbId>) -> Self {
        Self {
            mode: mode,
            item: item
        }
    }
}