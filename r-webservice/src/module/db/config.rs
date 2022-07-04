use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::ModuleDBEscape;
use crate::r#type::ServiceError;

pub use r_webservice_macro::ModuleDBMeta;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ModuleDBType {
    ARRAY(Box<ModuleDBType>),
    NULLABLE(Box<ModuleDBType>),
    SERIAL,
    BIG_SERIAL,
    INT,
    BIG_INT,
    TEXT,
    OBJECT
}

impl ModuleDBType {
    pub fn insartable(&self) -> bool {
        match self {
            Self::SERIAL|Self::BIG_SERIAL => {
                false
            },
            _ => {
                true
            }
        }
    }
}

#[allow(dead_code)]
pub struct ModuleDBConfig {
    table_name:   String,
    primary_key:  String,
    table_column: HashMap<String, ModuleDBType>
}

impl ModuleDBConfig {
    pub fn factory(
        table_name:   String,
        primary_key:  String,
        table_column: HashMap<String, ModuleDBType>
    ) -> Result<Self, ServiceError> {
        Ok(
            Self{
                table_name:   table_name.try_escape_to_string(())?,
                primary_key:  primary_key.try_escape_to_string(())?,
                table_column: table_column
            }
        )
    }

    pub fn get_table_name(&self) -> String {
        self.table_name.clone()
    }

    pub fn get_primary_key(&self) -> String {
        self.primary_key.clone()
    }

    pub fn get_columns(self) -> HashMap<String, ModuleDBType> {
        self.table_column
    }
}

pub trait ModuleDBMeta {
    fn config() -> ModuleDBConfig;
}