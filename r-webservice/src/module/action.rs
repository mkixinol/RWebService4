use sqlx::{FromRow, types::Json};
use serde::{Deserialize, Serialize};

use async_trait::async_trait;

use crate::r#type::{UDbId, UDbFlg};
use crate::module::{ModuleAction, ModuleDbAccess, ModuleDBMeta, ModuleDBType, TemplateTree};

#[derive(Clone, Debug, FromRow, ModuleDBMeta, Serialize, Deserialize)]
#[meta(table_name = "t_action")]
#[meta(primary_key = "id")]
pub struct Action {
    //基本項目
    #[meta(Type = ModuleDBType::BIG_SERIAL)]
    pub id: UDbId,
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub mode: UDbFlg,

    //親項目
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub page: UDbId, // Pageは階層構造を持つ

    //子項目
    #[meta(Type = ModuleDBType::OBJECT)]
    pub template: Json<TemplateTree>, // 使用するテンプレート

    //独自項目
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub method: UDbFlg, // 利用できるメソッド
    #[meta(Type = ModuleDBType::ARRAY(Box::new(ModuleDBType::TEXT)))]
    pub origin: Vec<String>, // 許可するOrigin
    #[meta(Type = ModuleDBType::ARRAY(Box::new(ModuleDBType::TEXT)))]
    pub content: Vec<String>, // 許可するContentType
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub format: UDbFlg, // ContentsType(HTML/JSON/File...)
    #[meta(Type = ModuleDBType::ARRAY(Box::new(ModuleDBType::BIG_INT)))]
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
    content TEXT[] NOT NULL,
    format BIGINT NOT NULL,
    auth BIGINT[] NOT NULL
)
*/

#[async_trait]
impl ModuleAction<Action> for Action {}
impl ModuleDbAccess<Action> for Action {}
