use serde::Serialize;
use serde_json::Value;
use async_trait::async_trait;
use futures_core::{stream::BoxStream};
use sqlx::{query_as, Executor, Postgres, postgres::PgRow, FromRow, error::Error};

use super::{ModuleDBSearchBuilder, ModuleDBOrderBuilder, ModuleDBMeta, ModuleDBEscape};

#[async_trait]
pub trait ModuleAction<T>: ModuleDbAccess<T> + ModuleDBMeta + Serialize
where
    T: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'static + ModuleDBMeta + Serialize
{
    async fn get<'a, DB>(
        pool: DB,
        search: &ModuleDBSearchBuilder
    ) -> Result<Option<T>, Error>
    where DB: Executor<'a, Database = Postgres> + 'a
    {
        let config = Self::config();
        let sql = format!(
            "SELECT * FROM {} {}",
            &config.get_table_name(),
            &search.to_where(),
        );

        Self::fetch_optional(pool, &sql).await
    }

    async fn list<'a, DB>(
        pool: DB,
        search: &ModuleDBSearchBuilder,
        order: &ModuleDBOrderBuilder
    ) -> Result<Vec<T>, Error>
    where DB: Executor<'a, Database = Postgres> + 'a
    {
        let config = Self::config();
        let sql = format!(
            "SELECT * FROM {} {} {}",
            &config.get_table_name(),
            &search.to_where(),
            &order.to_where()
        );
        Self::fetch_all(pool, &sql).await
    }

    async fn insert<'a, DB>(
        pool: DB,
        value: &Value
    ) -> Result<T, Error>
    where DB: Executor<'a, Database = Postgres> + 'a
    {
        let columns = Self::config();

        let table_name  = columns.get_table_name();
        let mut col_nm  = Vec::new();
        let mut col_val = Vec::new();
        for (key, types) in columns.get_columns() {
            if types.insartable() {
                col_nm.push(key.try_escape_to_string(()).unwrap());
                col_val.push(types.try_escape_to_string(value.get(&key).unwrap().clone()).unwrap());
            }
        }

        let sql = format!(
            "INSERT INTO {}({}) VALUES({}) RETURNING *",
            table_name,
            col_nm.join(","),
            col_val.join(",")
        );

        let result = Self::fetch_optional(pool, &sql).await?;
        Ok(result.unwrap())
    }

    async fn update<'a, DB>(
        pool: DB,
        value: &Value
    ) -> Result<T, Error>
    where DB: Executor<'a, Database = Postgres> + 'a
    {
        let columns = Self::config();

        let table_name  = columns.get_table_name();
        let primary_key = columns.get_primary_key();
        let mut primary_val = None;
        let mut cols = Vec::new();
        for (key, types) in columns.get_columns() {
            if key == primary_key {
                primary_val = Some(types.try_escape_to_string(value.get(&key).unwrap().clone()).unwrap());
            } else if types.insartable() {
                cols.push((
                    key.try_escape_to_string(()).unwrap(),
                    types.try_escape_to_string(value.get(&key).unwrap().clone()).unwrap()
                ));
            }
        }

        if let Some(key) = primary_val {
            let sql = format!(
                "UPDATE {} SET {} WHERE {} = {} RETURNING {} as id",
                table_name,
                cols.iter().map(|x| format!("{} = {}", x.0, x.1)).collect::<Vec<String>>().join(","),
                primary_key,
                key,
                primary_key
            );

            let result = Self::fetch_optional(pool, &sql).await?;
            Ok(result.unwrap())
        } else {
            let sql = format!("SELECT False");

            let result = Self::fetch_optional(pool, &sql).await?;
            Ok(result.unwrap())
        }
    }

    async fn delete<'a, DB>(
        pool: DB,
        value: &Value
    ) -> Result<T, Error>
    where DB: Executor<'a, Database = Postgres> + 'a
    {
        let columns = Self::config();

        let table_name  = columns.get_table_name();
        let primary_key = columns.get_primary_key();
        let mut primary_val = None;
        for (key, types) in columns.get_columns() {
            if key == primary_key {
                primary_val = Some(types.try_escape_to_string(value.get(&key).unwrap().clone()).unwrap());
            }
        }

        if let Some(key) = primary_val {
            let sql = format!(
                "DELETE FROM {} WHERE {} = {} RETURNING *",
                table_name,
                primary_key,
                key
            );
            let result = Self::fetch_optional(pool, &sql).await?;
            Ok(result.unwrap())
        } else {
            let sql = format!("SELECT False");

            let result = Self::fetch_optional(pool, &sql).await?;
            Ok(result.unwrap())
        }
    }
}

#[async_trait]
pub trait ModuleDbAccess<T>
where
    T: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'static
{
    fn fetch<'a, DB>(
        pool: DB,
        sql: &'a str
    ) -> BoxStream<'a, Result<T, Error>>
    where DB: Executor<'a, Database = Postgres> + 'a
    {
        query_as::<Postgres, T>(&sql).fetch(pool)
    }

    async fn fetch_all<'a, DB>(
        pool: DB,
        sql: &str
    ) -> Result<Vec<T>, Error> 
    where DB: Executor<'a, Database = Postgres> + 'a
    {
        query_as::<Postgres, T>(&sql).fetch_all(pool).await
    }

    async fn fetch_optional<'a, DB>(
        pool: DB,
        sql: &str
    ) -> Result<Option<T>, Error>
    where DB: Executor<'a, Database = Postgres> + 'a
    {
        query_as::<Postgres, T>(&sql).fetch_optional(pool).await
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
    T: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'static,
    T: ModuleDBMeta + Serialize
{
    /*
    async fn list_from_parent(id: UDbId) -> Vec<T> {
        T::list(vec![id]).await
    }
    */
}
