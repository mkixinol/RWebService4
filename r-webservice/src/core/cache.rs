use std::collections::HashMap;

use crate::module::{Auth, Template, TemplateTree};
use crate::r#type::UDbId;

#[derive(Clone, Debug)]
pub struct RWCache{
    auth: HashMap<UDbId, Auth>,
    tree: HashMap<UDbId, TemplateTree>,
    template: HashMap<UDbId, Template>,
}

impl RWCache {
    pub fn new() -> Self {
        Self {
            auth: HashMap::new(),
            tree: HashMap::new(),
            template: HashMap::new(),
        }
    }

    pub fn insert_auth(&mut self, key: &UDbId, auth: Auth) -> &mut Self {
        self.auth.insert(*key, auth);
        self
    }
    pub fn insert_tree(&mut self, key: &UDbId, tree: TemplateTree) -> &mut Self {
        self.tree.insert(*key, tree);
        self
    }
    pub fn insert_template(&mut self, key: &UDbId, template: Template) -> &mut Self {
        self.template.insert(*key, template);
        self
    }

    pub fn set_auth(&mut self, auth: HashMap<UDbId, Auth>) -> &mut Self {
        self.auth = auth;
        self
    }
    pub fn set_tree(&mut self, tree: HashMap<UDbId, TemplateTree>) -> &mut Self {
        self.tree = tree;
        self
    }
    pub fn set_template(&mut self, template: HashMap<UDbId, Template>) -> &mut Self {
        self.template = template;
        self
    }

    pub fn get_auth(&self, id: &UDbId) -> Option<&Auth> {
        self.auth.get(id)
    }
    pub fn get_tree(&self, id: &UDbId) -> Option<&TemplateTree> {
        self.tree.get(id)
    }
    pub fn get_template(&self, id: &UDbId) -> Option<&Template> {
        self.template.get(id)
    }
}
