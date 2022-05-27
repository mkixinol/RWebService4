mod sub;
mod page;
mod action;
pub mod process;
pub mod template;
pub mod content;

//pub use sub::PageAction;
pub use page::Page;
pub use action::Action;
pub use template::Template;

use std::collections::HashMap;
use async_trait::async_trait;
use futures_core::{Future, stream::BoxStream};
use sqlx::{query_as, PgPool, Postgres, postgres::PgRow, FromRow, error::Error};

use crate::r#type::UDbId;

pub enum ModuleDBType {
    INT(UDbId),
    INT_ARRAY(UDbId),
    TEXT(String),
    TEXT_ARRAY(String)
}

pub struct ModuleDBSearchBuilder(ModuleDBConfig, ModuleDBSearch);
pub enum ModuleDBSearch {
    OR(Vec<ModuleDBSearch>),
    AND(Vec<ModuleDBSearch>),
    NOT(Vec<ModuleDBSearch>),
    EQUAL((String, ModuleDBType)),
    GREATER((String, ModuleDBType)),
    GREATER_EQ((String, ModuleDBType)),
    LESSER((String, ModuleDBType)),
    LESSER_EQ((String, ModuleDBType)),
    LIKE((String, ModuleDBType))
}

pub struct ModuleDBConfig {
    table_name: String,
    table_column: HashMap<String, ModuleDBType>
}

pub struct ModuleDBOrderBuilder(ModuleDBConfig, ModuleDBOrder, ModuleDBPager);
pub enum ModuleDBOrder {
    DESC(String),
    ASC(String),
    ARRAY(Vec<ModuleDBOrder>)
}
pub enum ModuleDBPager {
    LIMIT(u64),
    OFFSET(u64),
    ARRAY(Vec<ModuleDBPager>)
}

#[async_trait]
pub trait ModuleAction<T>: ModuleDbAccess<T>
where
    T: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'static
{
    async fn get(pool: &'static PgPool, search: &ModuleDBSearchBuilder)
    -> Result<Option<T>, Error> {
        let config = Self::config();
        let sql = format!(
            "SELECT * FROM {} {}",
            &config.table_name,
            &search.to_where(),
        );
        Self::fetch_optional(pool, &sql).await
    }
    async fn list(pool: &'static PgPool, search: &ModuleDBSearchBuilder, order: &ModuleDBOrderBuilder)
    -> Result<Vec<T>, Error> {
        let config = Self::config();
        let sql = format!(
            "SELECT * FROM {} {} {}",
            &config.table_name,
            &search.to_where(),
            &order.to_where()
        );
        Self::fetch_all(pool, &sql).await
    }
    async fn insert(&self, pool: &'static PgPool) -> UDbId {
        3
    }
    async fn update(&self, pool: &'static PgPool, search: &ModuleDBSearchBuilder) -> bool {
        false
    }
    async fn delete(&self, pool: &'static PgPool, search: &ModuleDBSearchBuilder) -> bool {
        false
    }
}

#[async_trait]
pub trait ModuleDbAccess<T>
where
    T: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'static
{
    fn config() -> ModuleDBConfig;

    fn fetch<'a>(pool: &'a PgPool, sql: &'a str)
     -> BoxStream<'a, Result<T, Error>>{
        query_as::<Postgres, T>(sql).fetch(pool)
    }

    async fn fetch_all<'a>(pool: &'a PgPool, sql: &'a str)
     -> Result<Vec<T>, Error> {
        query_as::<Postgres, T>(sql).fetch_all(pool).await
    }

    async fn fetch_optional<'a>(pool: &'a PgPool, sql: &'a str)
     -> Result<Option<T>, Error> {
        query_as::<Postgres, T>(sql).fetch_optional(pool).await
    }
}

pub trait ModuleActionFromChild<T: ModuleAction<T>>
where
    T: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'static
{
    /*
    fn get_from_child<_U: ModuleAction<_U>>(id: &UDbId) -> T {
        T::get(*id).unwrap()
    }
    */
}

#[async_trait]
pub trait ModuleActionFromParent<T: ModuleAction<T>>
where
    T: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'static
{
    /*
    async fn list_from_parent(id: UDbId) -> Vec<T> {
        T::list(vec![id]).await
    }
    */
}

impl ModuleDBSearchBuilder {
    pub fn to_where(&self) -> String {
        "".to_string()
    }
}

impl ModuleDBOrderBuilder {
    pub fn to_where(&self) -> String {
        "".to_string()
    }
}