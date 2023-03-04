use crate::bridged_type::{BridgeableType, BridgedType, TypePosition};
use crate::{TypeDeclarations, SWIFT_BRIDGE_PREFIX};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, format_ident};
use syn::Path;

/// Rust: Result<T, E>
/// Swift: RustResult<T, E>
///
/// We don't use Swift's `Result` type since when we tried we saw a strange error
///  `'Sendable' class 'ResultTestOpaqueRustType' cannot inherit from another class other than 'NSObject'`
///  which meant that we could not use the `public class ResultTestOpaqueRustType: ResultTestOpaqueRustTypeRefMut {`
///  pattern that we use to prevent calling mutable methods on immutable references.
///  We only saw this error after `extension: ResultTestOpaqueRustType: Error {}` .. which was
///  necessary because Swift's Result type requires that the error implements the `Error` protocol.
#[derive(Debug)]
pub(crate) struct BuiltInResult {
    pub ok_ty: Box<dyn BridgeableType>,
    pub err_ty: Box<dyn BridgeableType>,
    ok_ty_string: String, 
    err_ty_string: String, 
}

impl BuiltInResult {
    pub(super) fn to_ffi_compatible_rust_type(&self, swift_bridge_path: &Path) -> TokenStream {
        if self.is_custom_result_type() {
            let ty = format_ident!("Result{}And{}", self.ok_ty_string, self.err_ty_string);
            return quote!{
                #ty
            }
        }
        // TODO: Choose the kind of Result representation based on whether or not the ok and error
        //  types are primitives.
        //  See `swift-bridge/src/std_bridge/result`
        let result_kind = if self.ok_ty.can_be_encoded_with_zero_bytes() {
            quote! {
                ResultVoidAndPtr
            }
        } else {
            quote! {
                ResultPtrAndPtr
            }
        };

        quote! {
            #swift_bridge_path::result::#result_kind
        }
    }

    pub(super) fn convert_rust_expression_to_ffi_type(
        &self,
        expression: &TokenStream,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
        span: Span,
    ) -> TokenStream {
        let convert_ok = self.ok_ty.convert_rust_expression_to_ffi_type(
            &quote! { ok },
            swift_bridge_path,
            types,
            span,
        );

        let convert_err = self.err_ty.convert_rust_expression_to_ffi_type(
            &quote! { err },
            swift_bridge_path,
            types,
            span,
        );

        if self.is_custom_result_type() {
            if self.err_ty.can_be_encoded_with_zero_bytes() {
                todo!();
            }
            if self.ok_ty.can_be_encoded_with_zero_bytes() {
                let ffi_enum_name = self.to_ffi_compatible_rust_type(swift_bridge_path);
                let err_ffi = self.err_ty.convert_rust_expression_to_ffi_type(
                    &quote!(err),
                    swift_bridge_path,
                    types,
                    span,
                );
                return quote! {
                    match #expression {
                        Ok(ok) => #ffi_enum_name::Ok(std::ptr::null_mut::<std::ffi::c_void>()),
                        Err(err) => #ffi_enum_name::Err(#err_ffi),
                    }
                };
            } 
            let ffi_enum_name = self.to_ffi_compatible_rust_type(swift_bridge_path);
            let ok_ffi =
                self.ok_ty.convert_rust_expression_to_ffi_type(&quote!(ok), swift_bridge_path, types, span);
            let err_ffi = self.err_ty.convert_rust_expression_to_ffi_type(
                &quote!(err),
                swift_bridge_path,
                types,
                span,
            );
            return quote! {
                match #expression {
                    Ok(ok) => #ffi_enum_name::Ok(#ok_ffi),
                    Err(err) => #ffi_enum_name::Err(#err_ffi),
                }
            };
        }

        if self.ok_ty.can_be_encoded_with_zero_bytes() {
            quote! {
                match #expression {
                    Ok(ok) => {
                        #swift_bridge_path::result::ResultVoidAndPtr {
                            is_ok: true,
                            err: std::ptr::null_mut::<std::ffi::c_void>()
                        }
                    }
                    Err(err) => {
                        #swift_bridge_path::result::ResultVoidAndPtr {
                            is_ok: false,
                            err: #convert_err as *mut std::ffi::c_void
                        }
                    }
                }
            }
        } else {
            quote! {
                match #expression {
                    Ok(ok) => {
                        #swift_bridge_path::result::ResultPtrAndPtr {
                            is_ok: true,
                            ok_or_err: #convert_ok as *mut std::ffi::c_void
                        }
                    }
                    Err(err) => {
                        #swift_bridge_path::result::ResultPtrAndPtr {
                            is_ok: false,
                            ok_or_err: #convert_err as *mut std::ffi::c_void
                        }
                    }
                }
            }
        }
    }

    pub(super) fn convert_ffi_value_to_rust_value(
        &self,
        expression: &TokenStream,
        span: Span,
        swift_bridge_path: &Path,
        types: &TypeDeclarations,
    ) -> TokenStream {
        let convert_ok = self.ok_ty.convert_ffi_result_ok_value_to_rust_value(
            expression,
            swift_bridge_path,
            types,
        );

        let convert_err = self.err_ty.convert_ffi_result_err_value_to_rust_value(
            expression,
            swift_bridge_path,
            types,
        );

        quote_spanned! {span=>
            if #expression.is_ok {
                std::result::Result::Ok(#convert_ok)
            } else {
                std::result::Result::Err(#convert_err)
            }
        }
    }

    pub fn to_rust_type_path(&self, types: &TypeDeclarations) -> TokenStream {
        let ok = self.ok_ty.to_rust_type_path(types);
        let err = self.err_ty.to_rust_type_path(types);

        quote! { Result<#ok, #err> }
    }

    pub fn to_swift_type(&self, type_pos: TypePosition, types: &TypeDeclarations) -> String {
        match type_pos {
            TypePosition::FnReturn(_) => self.ok_ty.to_swift_type(type_pos, types),
            TypePosition::FnArg(_, _) | TypePosition::SharedStructField => {
                format!(
                    "RustResult<{}, {}>",
                    self.ok_ty.to_swift_type(type_pos, types),
                    self.err_ty.to_swift_type(type_pos, types),
                )
            }
            TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => {
                "__private__ResultPtrAndPtr".to_string()
            }
        }
    }

    pub fn convert_ffi_value_to_swift_value(
        &self,
        expression: &str,
        type_pos: TypePosition,
        types: &TypeDeclarations,
    ) -> String {
        if self.is_custom_result_type() {
            if self.err_ty.can_be_encoded_with_zero_bytes() {
                todo!();
            }
            let c_ok_name  = format!("{}$ResultOk", SWIFT_BRIDGE_PREFIX);
            let c_err_name =  format!("{}$ResultErr", SWIFT_BRIDGE_PREFIX);
            let ok_swift_type = if self.ok_ty.can_be_encoded_with_zero_bytes() {
                "".to_string()
            } else {
                " ".to_string() + &self.ok_ty.convert_ffi_expression_to_swift_type("val.payload.ok", type_pos, types)
            };
            let err_swift_type =
                self.err_ty.convert_ffi_expression_to_swift_type("val.payload.err", type_pos, types);
    
            return match type_pos {
                TypePosition::FnArg(_, _) => todo!(),
                TypePosition::FnReturn(_) => format!(
                        "try {{
        let val = {expression};
        switch val.tag {{
        case {c_ok_name}:
            return{ok_swift_type}
        case {c_err_name}:
            throw {err_swift_type}
        default:
            fatalError()
    }} }}()",
                    expression = expression,
                    c_ok_name = c_ok_name,
                    c_err_name = c_err_name,
                    ok_swift_type = ok_swift_type,
                    err_swift_type = err_swift_type
                ),
                TypePosition::SharedStructField => todo!(),
                TypePosition::SwiftCallsRustAsyncOnCompleteReturnTy => todo!(),
            };
        }
        let (mut ok, err) = if let Some(zero_byte_encoding) = self.ok_ty.only_encoding() {
            let ok = zero_byte_encoding.swift;
            let convert_err = self
                .err_ty
                .convert_ffi_expression_to_swift_type("val.err!", type_pos, types);

            (ok, convert_err)
        } else {
            let convert_ok =
                self.ok_ty
                    .convert_ffi_expression_to_swift_type("val.ok_or_err!", type_pos, types);
            let convert_err =
                self.err_ty
                    .convert_ffi_expression_to_swift_type("val.ok_or_err!", type_pos, types);

            (convert_ok, convert_err)
        };

        // There is a Swift compiler bug in Xcode 13 where using an explicit `()` here somehow leads
        // the Swift compiler to a compile time error:
        // "Unable to infer complex closure return type; add explicit type to disambiguate"
        //
        // It's asking us to add a `{ () -> () in .. }` explicit type to the beginning of our closure.
        //
        // To solve this bug we can either add that explicit closure type, or remove the explicit
        // `return ()` in favor of a `return`.. Not sure why making the return type less explicit
        //  solves the compile time error.. But it does..
        //
        // As mentioned, this doesn't seem to happen in Xcode 14.
        // So, we can remove this if statement whenever we stop supporting Xcode 13.
        if self.ok_ty.is_null() {
            ok = "".to_string();
        }

        format!(
            "try {{ let val = {expression}; if val.is_ok {{ return {ok} }} else {{ throw {err} }} }}()",
            expression = expression,
            err = err
        )
    }

    pub fn convert_swift_expression_to_ffi_compatible(
        &self,
        expression: &str,
        type_pos: TypePosition,
    ) -> String {
        let convert_ok = self
            .ok_ty
            .convert_swift_expression_to_ffi_type("ok", type_pos);
        let convert_err = self
            .err_ty
            .convert_swift_expression_to_ffi_type("err", type_pos);

        if self.ok_ty.can_be_encoded_with_zero_bytes() {
            format!(
                "{{ switch {val} {{ case .Ok(let ok): return __private__ResultVoidAndPtr(is_ok: true, err: nil) case .Err(let err): return __private__ResultVoidAndPtr(is_ok: false, err: {convert_err}) }} }}()",
                val = expression
            )
        } else {
            format!(
                "{{ switch {val} {{ case .Ok(let ok): return __private__ResultPtrAndPtr(is_ok: true, ok_or_err: {convert_ok}) case .Err(let err): return __private__ResultPtrAndPtr(is_ok: false, ok_or_err: {convert_err}) }} }}()",
                val = expression
            )
        }
    }

    pub fn to_c(&self) -> String {
        if self.is_custom_result_type() {
            return format!("struct {}$Result{}And{}", SWIFT_BRIDGE_PREFIX, self.ok_ty_string, self.err_ty_string);
        }
        // TODO: Choose the kind of Result representation based on whether or not the ok and error
        //  types are primitives.
        //  See `swift-bridge/src/std_bridge/result`
        if self.ok_ty.can_be_encoded_with_zero_bytes() {
            format!("struct __private__ResultVoidAndPtr")
        } else {
            format!("struct __private__ResultPtrAndPtr")
        }
    }

    pub fn generate_ffi_definition(&self, swift_bridge_path: &Path, types: &TypeDeclarations) -> Option<TokenStream> {
        if !(self.ok_ty.is_passed_via_pointer() && self.err_ty.is_passed_via_pointer()) {
            if self.err_ty.can_be_encoded_with_zero_bytes() {
                todo!()
            }
            let ty = self.to_ffi_compatible_rust_type(swift_bridge_path);
            let ok = if self.ok_ty.can_be_encoded_with_zero_bytes() {
                quote!{* mut std :: ffi :: c_void}
            } else {
                self.ok_ty.to_ffi_compatible_rust_type(swift_bridge_path, types)
            };
            
            let err = self.err_ty.to_ffi_compatible_rust_type(swift_bridge_path, types);
            return Some(quote!{
                #[repr(C)]
                pub enum #ty {
                    Ok(#ok), 
                    Err(#err),
                }
            });
        }
        return None;
    }

    pub fn generate_c_declaration(&self) -> Option<String> {
        if self.is_custom_result_type() {
            if self.err_ty.can_be_encoded_with_zero_bytes() {
                todo!();
            }
            let c_type = format!("{}$Result{}And{}", SWIFT_BRIDGE_PREFIX, self.ok_ty_string, self.err_ty_string);
            let c_enum_name = c_type.clone();
            let c_tag_name = format!("{}$Tag", c_type.clone());
            let c_fields_name = format!("{}$Fields", c_type);

            let ok_c_field_name = if self.ok_ty.can_be_encoded_with_zero_bytes() {
                "".to_string()
            } else {
                format!("{} ok; ", self.ok_ty.to_c_type())
            };
            let err_c_field_name = self.err_ty.to_c_type();
            let ok_c_tag_name = format!("{}$ResultOk", SWIFT_BRIDGE_PREFIX);
            let err_c_tag_name = format!("{}$ResultErr", SWIFT_BRIDGE_PREFIX);

            return Some(format!("typedef enum {c_tag_name} {{{ok_c_tag_name}, {err_c_tag_name}}} {c_tag_name};
union {c_fields_name} {{{ok_c_field_name}{err_c_field_name} err;}};
typedef struct {c_enum_name}{{{c_tag_name} tag; union {c_fields_name} payload;}} {c_enum_name};",
            c_enum_name = c_enum_name, c_tag_name = c_tag_name, c_fields_name = c_fields_name, ok_c_field_name = ok_c_field_name, err_c_field_name = err_c_field_name, ok_c_tag_name = ok_c_tag_name, err_c_tag_name = err_c_tag_name,
));   
        }
        return None;
    }

    fn is_custom_result_type(&self) -> bool {
        // ResultPtrAndPtr
        if self.ok_ty.is_passed_via_pointer() && self.err_ty.is_passed_via_pointer() {
            return false;
        }

        // ResultVoidAndPtr or ResultPtrAndVoid
        if (self.ok_ty.only_encoding().is_some() && self.err_ty.is_passed_via_pointer()) || (self.ok_ty.is_passed_via_pointer() && self.err_ty.only_encoding().is_some()) {
            return false;
        }

        return true;
    } 
}

impl BuiltInResult {
    /// Go from `Result < A , B >` to a `BuiltInResult`.
    pub fn from_str_tokens(string: &str, types: &TypeDeclarations) -> Option<Self> {
        // A , B >
        let trimmed = string.trim_start_matches("Result < ");
        // A , B
        let trimmed = trimmed.trim_end_matches(" >");

        // [A, B]
        let mut ok_and_err = trimmed.split(",");
        let ok = ok_and_err.next()?.trim();
        let err = ok_and_err.next()?.trim();

        let ok_ty_string = if ok == "()" {
            format!("Void")
        } else {
            format!("{}", ok)
        };
        let err_ty_string = format!("{}", err);

        let ok = BridgedType::new_with_str(ok, types)?;
        let err = BridgedType::new_with_str(err, types)?;

        Some(BuiltInResult {
            ok_ty: Box::new(ok),
            err_ty: Box::new(err),
            ok_ty_string,
            err_ty_string,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    /// Verify that we can parse a `Result<(), ()>`
    #[test]
    fn result_from_null_type() {
        let tokens = quote! { Result<(), ()> }.to_token_stream().to_string();

        let result = BuiltInResult::from_str_tokens(&tokens, &TypeDeclarations::default()).unwrap();

        assert!(result.ok_ty.is_null());
        assert!(result.err_ty.is_null());
    }
}
