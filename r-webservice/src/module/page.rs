use sqlx::FromRow;
use serde::{Serialize, Deserialize};

use async_trait::async_trait;

use crate::r#type::{UDbId, UDbFlg};
use crate::module::{ModuleAction, ModuleDbAccess, ModuleDBMeta, ModuleDBType};

#[derive(Clone, Debug, FromRow, ModuleDBMeta, Serialize, Deserialize)]
#[meta(table_name = "t_page")]
#[meta(primary_key = "id")]
pub struct Page {
    //基本項目
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub id: UDbId,
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub mode: UDbFlg,

    //親項目
    #[meta(Type = ModuleDBType::BIG_INT)]
    pub page: UDbId, // Pageは階層構造を持つ

    //独自項目
    #[meta(Type = ModuleDBType::TEXT)]
    pub path: String, // URI
    #[meta(Type = ModuleDBType::ARRAY(Box::new(ModuleDBType::TEXT)))]
    pub param: Vec<String>, // URI内に含まれている正規表現で抽出されるパラメータ
}

/*
CREATE TABLE t_page (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    mode BIGINT NOT NULL,
    page BIGINT NOT NULL,
    path TEXT NOT NULL,
    param TEXT[] NOT NULL
)
*/

#[async_trait]
impl ModuleAction<Page> for Page {}
impl ModuleDbAccess<Page> for Page {}