use crate::BridgedType;
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct TypeDeclarations {
    decls: HashMap<String, BridgedType>,
    order: Vec<String>,
}

impl TypeDeclarations {}

impl TypeDeclarations {}

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

    pub fn order(&self) -> &Vec<String> {
        &self.order
    }
}
