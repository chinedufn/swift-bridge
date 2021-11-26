use crate::BridgedType;
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct TypeDeclarations {
    decls: HashMap<String, BridgedType>,
    order: Vec<String>,
}

impl TypeDeclarations {
    pub(crate) fn get(&self, type_name: &String) -> Option<&BridgedType> {
        self.decls.get(type_name)
    }

    pub(crate) fn insert(&mut self, type_name: String, ty: BridgedType) {
        self.decls.insert(type_name.clone(), ty);
        self.order.push(type_name);
    }

    pub(crate) fn contains_key(&self, type_name: &String) -> bool {
        self.decls.contains_key(type_name)
    }

    pub fn types(&self) -> Vec<&BridgedType> {
        self.order
            .iter()
            .map(|ty| self.decls.get(ty).unwrap())
            .collect()
    }
}
