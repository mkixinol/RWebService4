mod module;
mod request;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use sqlx::PgPool;
use actix_web::HttpRequest;
use actix_session::Session;

use crate::r#type::{IDbNo, UDbId, ServiceError, ServerError};
use crate::core::cache::RWCache;
use crate::module::*;
use module::EngineModuleInner;
use request::EngineRequest;

macro_rules! define_module_wrap {
    ($($module_name:ident),+) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub enum EngineModule {
            $($module_name($module_name),)+
        }
        pub trait EngineModuleWrap {
            fn wrap_engine_module(self) -> EngineModule;
        }
        $(impl_wrap_engine_module!{$module_name})+
    };
}

macro_rules! impl_wrap_engine_module {
    ($module_name:ident) => {
        impl EngineModuleWrap for $module_name {
            fn wrap_engine_module(self) -> EngineModule {
                EngineModule::$module_name(self)
            }
        }
    };
}

impl Template {
    pub async fn execute<'a>(
        &self,
        pool: &'a PgPool,
        cache: &'a RWCache,
        option: &Option<HashMap<IDbNo, UDbId>>,
        request: &HttpRequest,
        session: &Session,
        //body: &RequestBody,
        value: &[EngineValue]
    ) -> Result<(bool, EngineValue), ServiceError> {
        match self.engine {
            EngineValue::ENGINE_TEMPLATE_REQUEST => {
                EngineRequest::new().execute(
                    pool,
                    cache,
                    option,
                    &self,
                    request,
                    session,
                //    body
                ).await
            },
            EngineValue::ENGINE_MODULE_PAGE => {
                EngineModuleInner::<Page>::new().execute(pool, &self.path, option, value).await
            },
            EngineValue::ENGINE_MODULE_ACTION => {
                EngineModuleInner::<Action>::new().execute(pool, &self.path, option, value).await
            },
            EngineValue::ENGINE_MODULE_TEMPLATE => {
                EngineModuleInner::<Template>::new().execute(pool, &self.path, option, value).await
            },
            _ => {
                Err(ServerError::TemplateModuleError.into())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EngineValue {
    Any(Value),
    SearchDetail(ModuleDBSearch),
    SearchList((ModuleDBSearch, ModuleDBOrder, ModuleDBPager)),
    Upsert(Value),
    Module(EngineModule),
    ModuleArray(Vec<EngineModule>),
    ServiceError(ServiceError),
    ServerError(ServerError),
    None
}

define_module_wrap!{Page, Action, Template}

impl EngineValue {
    pub const ENGINE_TEMPLATE_DEFAULT:  IDbNo = 0;

    pub const ENGINE_TEMPLATE_REQUEST:  IDbNo = -1;
    pub const ENGINE_TEMPLATE_V8:       IDbNo = -2;
    pub const ENGINE_TEMPLATE_TERA:     IDbNo = -3;

    pub const ENGINE_MODULE_CONTENT:    IDbNo = 1;
    pub const ENGINE_MODULE_MEMBER:     IDbNo = 2;
    pub const ENGINE_MODULE_AUTH:       IDbNo = 3;
    pub const ENGINE_MODULE_PAGE:       IDbNo = 4;
    pub const ENGINE_MODULE_ACTION:     IDbNo = 5;
    pub const ENGINE_MODULE_TEMPLATE:   IDbNo = 6;


    pub const ENGINE_TYPE_ANY:          &'static str = "Any";
    pub const ENGINE_TYPE_SEARCHDETAIL: &'static str = "SearchDetail";
    pub const ENGINE_TYPE_SEARCHLIST:   &'static str = "SearchList";
    pub const ENGINE_TYPE_UPSERT:       &'static str = "Upsert";
    pub const ENGINE_TYPE_MODULE:       &'static str = "Module";
    pub const ENGINE_TYPE_MODULEARRAY:  &'static str = "ModuleArray";
    pub const ENGINE_TYPE_SERVICEERROR: &'static str = "ServiceError";
    pub const ENGINE_TYPE_SERVERERROR:  &'static str = "ServerError";
    pub const ENGINE_TYPE_NONE:         &'static str = "None";
}
