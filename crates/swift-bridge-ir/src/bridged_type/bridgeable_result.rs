use crate::bridged_type::{BridgeableType, BridgedType, TypePosition};
use crate::TypeDeclarations;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
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
}

impl BuiltInResult {
    pub(super) fn to_ffi_compatible_rust_type(&self, swift_bridge_path: &Path) -> TokenStream {
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

    pub fn to_rust_type_path(&self) -> TokenStream {
        let ok = self.ok_ty.to_rust_type_path();
        let err = self.err_ty.to_rust_type_path();

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
        if let Some(zero_byte_encoding) = self.ok_ty.only_encoding() {
            let ok = zero_byte_encoding.swift;
            let convert_err = self
                .err_ty
                .convert_ffi_expression_to_swift_type("val.err!", type_pos, types);

            format!(
                "try {{ let val = {expression}; if val.is_ok {{ return {ok} }} else {{ throw {err} }} }}()",
                expression = expression,
                err = convert_err
            )
        } else {
            let convert_ok =
                self.ok_ty
                    .convert_ffi_expression_to_swift_type("val.ok_or_err!", type_pos, types);
            let convert_err =
                self.err_ty
                    .convert_ffi_expression_to_swift_type("val.ok_or_err!", type_pos, types);

            format!(
                "try {{ let val = {expression}; if val.is_ok {{ return {ok} }} else {{ throw {err} }} }}()",
                expression = expression,
                ok = convert_ok,
                err = convert_err
            )
        }
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

    pub fn to_c(&self) -> &'static str {
        // TODO: Choose the kind of Result representation based on whether or not the ok and error
        //  types are primitives.
        //  See `swift-bridge/src/std_bridge/result`
        if self.ok_ty.can_be_encoded_with_zero_bytes() {
            "struct __private__ResultVoidAndPtr"
        } else {
            "struct __private__ResultPtrAndPtr"
        }
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

        let ok = BridgedType::new_with_str(ok, types)?;
        let err = BridgedType::new_with_str(err, types)?;

        Some(BuiltInResult {
            ok_ty: Box::new(ok),
            err_ty: Box::new(err),
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
