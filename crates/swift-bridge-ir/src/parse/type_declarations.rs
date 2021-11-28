use crate::BridgedType;
use quote::ToTokens;
use std::collections::HashMap;
use std::ops::Deref;
use syn::{PatType, Type};

#[derive(Default)]
pub(crate) struct TypeDeclarations {
    decls: HashMap<String, BridgedType>,
    order: Vec<String>,
}

impl TypeDeclarations {
    pub(crate) fn get(&self, type_name: &String) -> Option<&BridgedType> {
        self.decls.get(type_name)
    }

    pub(crate) fn get_with_pat_type(&self, pat_ty: &PatType) -> Option<&BridgedType> {
        self.get_with_type(&pat_ty.ty)
    }

    pub(crate) fn get_with_type(&self, ty: &Type) -> Option<&BridgedType> {
        let ty = match ty.deref() {
            Type::Reference(reference) => reference.elem.to_token_stream().to_string(),
            Type::Path(path) => path.to_token_stream().to_string(),
            _ => todo!("Handle other cases"),
        };
        self.get(&ty)
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
