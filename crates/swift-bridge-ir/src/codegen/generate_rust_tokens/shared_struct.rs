//! More tests can be found in
//! crates/swift-bridge-ir/src/codegen/codegen_tests/shared_struct_codegen_tests.rs

use crate::bridged_type::{BridgedType, SharedStruct};
use crate::{SwiftBridgeModule, SWIFT_BRIDGE_PREFIX};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

impl SwiftBridgeModule {
    /// Generate the tokens for a shared struct.
    pub(super) fn generate_shared_struct_tokens(
        &self,
        shared_struct: &SharedStruct,
    ) -> Option<TokenStream> {
        if shared_struct.already_declared {
            return None;
        }

        let struct_name = &shared_struct.name;
        let swift_bridge_path = &self.swift_bridge_path;

        let repr_c_struct_name = format!("{}{}", SWIFT_BRIDGE_PREFIX, struct_name);
        let repr_c_struct_name = Ident::new(&repr_c_struct_name, struct_name.span());

        let struct_fields: Vec<TokenStream> = shared_struct
            .fields
            .normalized_fields()
            .iter()
            .map(|norm_field| {
                let maybe_name_and_colon = norm_field.maybe_name_and_colon();
                let ty = &norm_field.ty;

                quote! {
                    pub #maybe_name_and_colon #ty
                }
            })
            .collect();
        let struct_fields = shared_struct.fields.wrap_declaration_fields(&struct_fields);

        let repr_c_struct_fields: Vec<TokenStream> = shared_struct
            .fields
            .normalized_fields()
            .iter()
            .map(|norm_field| {
                let maybe_name_and_colon = norm_field.maybe_name_and_colon();
                let ty = &norm_field.ty;

                let ty = BridgedType::new_with_type(ty, &self.types).unwrap();
                let ty = ty.to_ffi_compatible_rust_type(&self.swift_bridge_path);

                quote! {
                    #maybe_name_and_colon #ty
                }
            })
            .collect();
        let repr_c_struct_fields = shared_struct
            .fields
            .wrap_declaration_fields(&repr_c_struct_fields);

        let convert_rust_to_ffi = shared_struct.convert_rust_expression_to_ffi_repr(
            &quote! { self },
            &self.types,
            &self.swift_bridge_path,
        );
        let convert_ffi_to_rust =
            shared_struct.convert_ffi_repr_to_rust(&quote! { self }, &self.types);

        let struct_ffi_repr = if shared_struct.fields.is_empty() {
            // Using a u8 is arbitrary... We just need a field since empty structs aren't FFI safe.
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct #repr_c_struct_name {
                    _private: u8
                }
            }
        } else {
            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct #repr_c_struct_name #repr_c_struct_fields
            }
        };

        let definition = quote! {
            pub struct #struct_name #struct_fields

            #struct_ffi_repr

            impl #swift_bridge_path::SharedStruct for #struct_name {
                type FfiRepr = #repr_c_struct_name;
            }

            impl #struct_name {
                #[doc(hidden)]
                #[inline(always)]
                pub fn into_ffi_repr(self) -> #repr_c_struct_name {
                    #convert_rust_to_ffi
                }
            }

            impl #repr_c_struct_name {
                #[doc(hidden)]
                #[inline(always)]
                pub fn into_rust_repr(self) -> #struct_name {
                    #convert_ffi_to_rust
                }
            }
        };

        Some(definition)
    }
}
