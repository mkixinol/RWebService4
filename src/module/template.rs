use sqlx::FromRow;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{query_as, PgPool};

use crate::r#type::{UDbId, UDbFlg, IDbNo, Engine};
use crate::module::{ModuleAction, ModuleDbAccess, ModuleDBConfig};

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Template {
    //基本項目
    pub id: UDbId,
    pub mode: UDbFlg,

    //管理項目
    pub subject: String,

    //独自項目
    pub engine: IDbNo, // テンプレートタイプ
    pub path:   String,
    pub option: String
}

/*
CREATE TABLE t_template (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    mode BIGINT NOT NULL,
    subject TEXT NOT NULL,
    engine BIGINT NOT NULL,
    path TEXT NOT NULL,
    param TEXT NOT NULL
)
*/

#[async_trait]
impl ModuleAction<Template> for Template {
    /*
    async fn list(_ids: Vec<UDbId>) -> Vec<Template> {
        let mut list = Vec::new();
        list.push(
            Template{
                id: 1,
                mode: 0,
                subject: String::from("/template/v8/1.js"),
                engine: Engine::ENGINE_TYPE_V8,
                path: String::from("/template/v8/1.js"),
                option: String::from("{}"),
            }
        );
        list
    }
    */
}

impl ModuleDbAccess<Template> for Template {
    fn config() -> ModuleDBConfig {
        ModuleDBConfig{
            table_name: "t_test".to_string(),
            table_column: HashMap::new()
        }
    }
}
