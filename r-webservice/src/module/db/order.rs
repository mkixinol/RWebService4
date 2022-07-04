use serde::{Serialize, Deserialize};

use super::{ModuleDBConfig};

pub struct ModuleDBOrderBuilder(ModuleDBConfig, ModuleDBOrder, ModuleDBPager);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModuleDBOrder {
    DESC(String),
    ASC(String),
    ARRAY(Vec<ModuleDBOrder>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModuleDBPager {
    LIMIT(u64),
    OFFSET(u64),
    ARRAY(Vec<ModuleDBPager>)
}

impl ModuleDBOrderBuilder {
    pub fn new(config: ModuleDBConfig, order: ModuleDBOrder, limit: ModuleDBPager) -> Self {
        Self(config, order, limit)
    }
    pub fn to_where(&self) -> String {
        "".to_string()
    }
}