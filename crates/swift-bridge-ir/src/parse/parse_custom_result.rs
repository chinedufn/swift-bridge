use crate::bridged_type::bridgeable_custom_result::CustomResultType;
use crate::bridged_type::BridgedType;
use crate::parse::TypeDeclarations;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::parse::Parse;
use syn::token::{Gt, Lt};
use syn::Token;
use syn::TypeParam;

#[derive(Clone)]
pub(crate) struct CustomResultTypeDeclaration {
    pub ty: Ident,
    pub ok: TypeParam,
    pub err: TypeParam,
}

impl CustomResultTypeDeclaration {
    pub fn to_bridged_type(&self) -> CustomResultType {
        CustomResultType {
            ty: self.ty.clone(),
            ok_ty: self.ok.clone(),
            err_ty: self.err.clone(),
        }
    }

    pub fn maybe_tokens_from_str(string: &str, types: &TypeDeclarations) -> Option<TokenStream> {
        let trimmed = string.to_string();
        let trimmed = trimmed.trim_start_matches("Result <");
        let trimmed = trimmed.trim_end_matches(">");
        let mut ok_and_err = trimmed.split(",");
        let ok_str = ok_and_err.next()?.trim();
        let err_str = ok_and_err.next()?.trim();
        let _ = BridgedType::new_with_str(ok_str, &types)?;
        let _ = BridgedType::new_with_str(err_str, &types)?;
        let ok = format_ident!("{}", ok_str);
        let err = format_ident!("{}", err_str);
        Some(quote! {
            Result<#ok, #err>
        })
    }
}

impl Parse for CustomResultTypeDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<CustomResultTypeDeclaration> {
        let ty: Ident = input.parse()?;
        let _: Lt = input.parse()?;
        let ok: TypeParam = input.parse()?;
        let _: Token![,] = input.parse()?;
        let err: TypeParam = input.parse()?;
        let _: Gt = input.parse()?;
        Ok(CustomResultTypeDeclaration { ty, ok, err })
    }
}
