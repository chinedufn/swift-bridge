use crate::bridged_type::bridgeable_custom_result::CustomResultType;
use proc_macro2::Ident;
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
    /***
    pub fn maybe_insert(tokens: &str, type_declarations: &mut TypeDeclarations) -> Option<String> {
        let trimmed = tokens.trim_start_matches("Result <");
        let trimmed = trimmed.trim_end_matches(">");
        let mut ok_and_err = trimmed.split(",");
        let ok_str = ok_and_err.next()?.trim();
        let err_str      = ok_and_err.next()?.trim();
        let result_type = format!("Result<{},{}>", ok_str, err_str);
        let ok  = BridgedType::new_with_str(ok_str, &type_declarations)?;
        let err = BridgedType::new_with_str(err_str, &type_declarations)?;
        if let (Some(_), Some(_)) = (ok, err) {
            let ok = format_ident!("{}", ok_str);
            let err = format_ident!("{}", err_str);
            let tokens = quote!{
                Result<#ok, #err>
            };
            let custom_result_type = syn::parse2::<CustomResultTypeDeclaration>(tokens)?;
            Ok(result_type)
        } else {
            Err(())
        }
    }
    ***/
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
