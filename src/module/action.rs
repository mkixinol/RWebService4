use sqlx::{FromRow, types::Json};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{query_as, PgPool};

use crate::r#type::{UDbId, UDbFlg};
use crate::module::{ModuleAction, ModuleDbAccess, ModuleDBConfig};

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Action {
    //基本項目
    pub id: UDbId,
    pub mode: UDbFlg,

    //親項目
    pub page: UDbId, // Pageは階層構造を持つ

    //子項目
    pub template: Json<Vec<ChildTemplate>>, // 使用するテンプレート

    //独自項目
    pub method: UDbFlg, // 利用できるメソッド
    pub origin: Vec<String>, // 許可するOrigin
    pub content: UDbFlg, // 許可するContentType
    pub format: UDbFlg, // ContentsType(HTML/JSON/File...)
    pub auth: Vec<UDbId>,
}

/*
CREATE TABLE t_action (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    mode BIGINT NOT NULL,
    page BIGINT NOT NULL,
    template JSONB NOT NULL,
    method BIGINT NOT NULL,
    origin TEXT[] NOT NULL
    content BIGINT NOT NULL,
    format BIGINT NOT NULL,
    auth BIGINT[] NOT NULL
)
*/

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct ChildTemplate {
    id: UDbId,
    method: Option<UDbFlg>,
    replace: Option<UDbId>,
    validate: Option<UDbId>,
}

#[async_trait]
impl ModuleAction<Action> for Action {
}

impl ModuleDbAccess<Action> for Action {
    fn config() -> ModuleDBConfig {
        ModuleDBConfig{
            table_name: "t_test".to_string(),
            table_column: HashMap::new()
        }
    }
}

impl ChildTemplate {
    pub fn id_list(&self) -> Vec<UDbId> {
        let mut ids = vec![self.id];
        if let Some(replace) = self.replace {
            ids.push(replace);
        }
        if let Some(validate) = self.validate {
            ids.push(validate);
        }
        ids
    }
}