use crate::built_in_types::ForeignBridgedType;
use quote::ToTokens;
use std::collections::HashMap;
use std::ops::Deref;
use syn::{PatType, Type};

#[derive(Default)]
pub(crate) struct TypeDeclarations {
    decls: HashMap<String, ForeignBridgedType>,
    order: Vec<String>,
}

impl TypeDeclarations {
    pub(crate) fn get(&self, type_name: &String) -> Option<&ForeignBridgedType> {
        self.decls.get(type_name)
    }

    pub(crate) fn get_with_pat_type(&self, pat_ty: &PatType) -> Option<&ForeignBridgedType> {
        self.get_with_type(&pat_ty.ty)
    }

    pub(crate) fn get_with_type(&self, ty: &Type) -> Option<&ForeignBridgedType> {
        let ty = match ty.deref() {
            Type::Reference(reference) => reference.elem.to_token_stream().to_string(),
            Type::Path(path) => path.to_token_stream().to_string(),
            _ => todo!("Handle other cases"),
        };
        self.get(&ty)
    }

    pub(crate) fn insert(&mut self, type_name: String, ty: ForeignBridgedType) {
        self.decls.insert(type_name.clone(), ty);
        self.order.push(type_name);
    }

    pub(crate) fn contains_key(&self, type_name: &String) -> bool {
        self.decls.contains_key(type_name)
    }

    pub fn types(&self) -> Vec<&ForeignBridgedType> {
        self.order
            .iter()
            .map(|ty| self.decls.get(ty).unwrap())
            .collect()
    }
}
