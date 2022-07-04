use std::collections::HashMap;
use serde::Serialize;
use serde_json::Value;
use sqlx::{PgPool, postgres::PgRow, FromRow};

use crate::r#type::{IDbNo, UDbId, ServiceError, ServerError};
use crate::module::*;
use super::{EngineValue, EngineModuleWrap};

pub struct EngineModuleInner<T>(Option<T>);
impl<T> EngineModuleInner<T>
where
    T: EngineModuleWrap,
    T: ModuleAction<T>,
    T: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'static + ModuleDBMeta + Serialize
{
    pub const ENGINE_FEATURE_DETAIL:     &'static str = "Detail";
    pub const ENGINE_FEATURE_LIST:       &'static str = "List";
    pub const ENGINE_FEATURE_INSERT:     &'static str = "Insert";
    pub const ENGINE_FEATURE_UPDATE:     &'static str = "Update";
    pub const ENGINE_FEATURE_BULKUPSERT: &'static str = "BulkUpsert";

    pub fn new() -> Self {
        Self(None)
    }

    pub async fn execute<'a>(
        &self,
        pool: &'a PgPool,
        path: &str,
        _option: &Option<HashMap<IDbNo, UDbId>>,
        value: &[EngineValue]
    ) -> Result<(bool, EngineValue), ServiceError> {
        match path {
            Self::ENGINE_FEATURE_DETAIL => {
                if let EngineValue::SearchDetail(search) = value.first().ok_or(
                    ServerError::TemplateRoutingError
                )? {
                    let item = T::get(
                        pool,
                        &ModuleDBSearchBuilder::new(T::config(), search.clone())
                    ).await;
                    match item {
                        Ok(o) => {
                            match o {
                                Some(s) => {
                                    Ok(
                                        (
                                            true,
                                            EngineValue::Module(
                                                s.wrap_engine_module()
                                            )
                                        )
                                    )
                                },
                                None => {
                                    Ok(
                                        (
                                            false,
                                            EngineValue::None
                                        )
                                    )
                                }
                            }
                        },
                        Err(_) => {
                            Err(ServerError::DBAccessError.into())
                        }
                    }
                } else {
                    Err(ServerError::TemplateTypeError.into())
                }
            },
            Self::ENGINE_FEATURE_LIST => {
                if let EngineValue::SearchList((search, order, pager)) = value.first().ok_or(
                    ServerError::TemplateRoutingError
                )? {
                    let item = T::list(
                        pool,
                        &ModuleDBSearchBuilder::new(T::config(), search.clone()),
                        &ModuleDBOrderBuilder::new(T::config(), order.clone(), pager.clone()),
                    ).await;
                    match item {
                        Ok(s) => {
                            Ok(
                                (
                                    true,
                                    EngineValue::ModuleArray(
                                        s.into_iter().map(|m| m.wrap_engine_module()).collect()
                                    )
                                )
                            )
                        },
                        Err(_) => {
                            Err(ServerError::DBAccessError.into())
                        }
                    }
                } else {
                    Err(ServerError::TemplateTypeError.into())
                }
            },
            Self::ENGINE_FEATURE_INSERT => {
                if let EngineValue::Upsert(value) = value.first().ok_or(
                    ServerError::TemplateRoutingError
                )? {
                    let item = T::insert(
                        pool,
                        &value
                    ).await;
                    match item {
                        Ok(s) => {
                            Ok(
                                (
                                    true,
                                    EngineValue::Module(
                                        s.wrap_engine_module()
                                    )
                                )
                            )
                        },
                        Err(_) => {
                            Err(ServerError::DBAccessError.into())
                        }
                    }
                } else {
                    Err(ServerError::TemplateTypeError.into())
                }
            },
            Self::ENGINE_FEATURE_UPDATE => {
                if let EngineValue::Upsert(value) = value.first().ok_or(
                    ServerError::TemplateRoutingError
                )? {
                    let item = T::update(
                        pool,
                        &value
                    ).await;
                    match item {
                        Ok(s) => {
                            Ok(
                                (
                                    true,
                                    EngineValue::Module(
                                        s.wrap_engine_module()
                                    )
                                )
                            )
                        },
                        Err(_) => {
                            Err(ServerError::DBAccessError.into())
                        }
                    }
                } else {
                    Err(ServerError::TemplateTypeError.into())
                }
            },
            Self::ENGINE_FEATURE_BULKUPSERT => {
                if let EngineValue::Upsert(Value::Array(value)) = value.first().ok_or(
                    ServerError::TemplateRoutingError
                )? {
                    let mut cn = pool.begin().await.unwrap();
                    let mut result = Vec::new();
                    for val in value {
                        let item = T::update(
                            &mut cn,
                            &val
                        ).await;
                        match item {
                            Ok(s) => {
                                result.push(s.wrap_engine_module());
                            },
                            Err(_) => {
                                let _ = cn.rollback().await;
                                return Err(ServerError::DBAccessError.into());
                            }
                        }
                    }
                    let _ = cn.commit().await;
                    Ok(
                        (
                            true,
                            EngineValue::ModuleArray(result)
                        )
                    )
                } else {
                    Err(ServerError::TemplateTypeError.into())
                }
            },
            _ => {
                Err(ServerError::TemplateModuleError.into())
            }
        }
    }
}
