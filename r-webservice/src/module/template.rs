use std::collections::HashMap;
use sqlx::{FromRow, types::Json};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::r#type::{UDbId, UDbFlg, IDbNo};
use crate::module::{ModuleAction, ModuleDbAccess, ModuleDBMeta, ModuleDBType};

#[derive(Clone, Debug, FromRow, ModuleDBMeta, Serialize, Deserialize)]
#[meta(table_name  = "t_template")]
#[meta(primary_key = "id")]
pub struct Template {
    //基本項目
    #[meta(Type = ModuleDBType::BIG_SERIAL)]
    pub id: UDbId,
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub mode: UDbFlg,

    //管理項目
    #[meta(Type = ModuleDBType::TEXT)]
    pub subject: String,

    //独自項目
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub engine: IDbNo, // テンプレートタイプ
    #[meta(Type = ModuleDBType::TEXT)]
    pub path:   String,
    #[meta(Type = ModuleDBType::OBJECT)]
    pub option: Json<HashMap<String, String>>
}

/*
CREATE TABLE t_template (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    mode BIGINT NOT NULL,
    subject TEXT NOT NULL,
    engine BIGINT NOT NULL,
    path TEXT NOT NULL,
    option JSONB NOT NULL default '{}'
)
*/

#[async_trait]
impl ModuleAction<Template> for Template {}
impl ModuleDbAccess<Template> for Template {}
