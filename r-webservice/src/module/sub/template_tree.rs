use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::r#type::{UDbId, IDbNo};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateTree {
    id: Option<UDbId>,
    option: Option<HashMap<IDbNo, UDbId>>,
    success: Option<Box<TemplateTree>>,
    fail: Option<Box<TemplateTree>>,
}

impl TemplateTree {
    pub fn id(&self) -> Option<UDbId> {
        self.id.clone()
    }

    pub fn option(&self) -> Option<HashMap<IDbNo, UDbId>> {
        self.option.clone()
    }

    pub fn success(&self) -> Option<&Self> {
        if let Some(s) = &self.success {
            Some(s)
        } else {
            None
        }
    }

    pub fn fail(&self) -> Option<&Self> {
        if let Some(s) = &self.fail {
            Some(s)
        } else {
            None
        }
    }

    pub fn id_list(&self) -> Vec<UDbId> {
        let mut ids = HashSet::new();
        if let Some(id) = self.id {
            ids.insert(id);
            if let Some(map) = &self.option {
                for (_, id) in map.iter() {
                    ids.insert(*id);
                }
            }
            if let Some(child) = &self.success {
                ids.extend(child.id_list());
            }
            if let Some(child) = &self.fail {
                ids.extend(child.id_list());
            }
        }
        ids.into_iter().collect()
    }
}