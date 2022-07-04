use serde::{Serialize, Deserialize};

use super::ModuleDBConfig;

pub struct ModuleDBSearchBuilder(ModuleDBConfig, ModuleDBSearch);
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ModuleDBSearch {
    OR(Vec<ModuleDBSearch>),
    AND(Vec<ModuleDBSearch>),
    NOT(Vec<ModuleDBSearch>),
    IN((String, String)),
    EQUAL((String, String)),
    GREATER((String, String)),
    GREATER_EQ((String, String)),
    LESSER((String, String)),
    LESSER_EQ((String, String)),
    LIKE((String, String))
}

impl ModuleDBSearchBuilder {
    pub fn new(config: ModuleDBConfig, search: ModuleDBSearch) -> Self {
        Self(config, search)
    }
    pub fn to_where(&self) -> String {
        "".to_string()
    }
}
