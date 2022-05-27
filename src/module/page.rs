use sqlx::FromRow;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{query_as, PgPool};

use crate::r#type::{UDbId, UDbFlg};
use crate::module::{ModuleAction, ModuleDbAccess, ModuleDBConfig};

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Page {
    //基本項目
    pub id: UDbId,
    pub mode: UDbFlg,

    //親項目
    pub page: UDbId, // Pageは階層構造を持つ

    //独自項目
    pub path: String, // URI
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

impl ModuleDbAccess<Page> for Page {
    fn config() -> ModuleDBConfig {
        ModuleDBConfig{
            table_name: "t_page".to_string(),
            table_column: HashMap::new()
        }
    }
}

#[async_trait]
impl ModuleAction<Page> for Page {
    /*
    async fn list(_ids: Vec<UDbId>) -> Vec<Page> {
        let mut list = Vec::new();
        list.push(
            Page{
                id: 1,
                mode: 0,
                page: 0,
                path: String::from("/a"),
                param: vec!(String::from("")),
            }
        );
        list.push(
            Page{
                id: 3, 
                mode: 0,
                page: 0, 
                path: String::from("/b{id:[0-9]+}"), 
                param: vec!(String::from("id")), 
            }
        );
        list.push(
            Page{
                id: 4, 
                mode: 0,
                page: 0, 
                path: String::from("/c{id:[0-9]+}/{act:(detail|list/all)}"), 
                param: vec!(String::from("id,act")), 
            }
        );
        list.push(
            Page{
                id: 9,
                mode: 0,
                page: 0,
                path: String::from("/files/{path:.+}"),
                param: vec!(String::from("path")),
            }
        );
        list
    }
    */
}

impl Page {
    /*
    pub async fn all() -> Vec<Page> {
        Page::list(vec![1,4,3]).await
    }
    */
}