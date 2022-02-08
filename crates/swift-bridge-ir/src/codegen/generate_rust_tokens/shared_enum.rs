//! More tests can be found in
//! crates/swift-bridge-ir/src/codegen/codegen_tests/shared_enum_codegen_tests.rs

use crate::bridged_type::SharedEnum;
use crate::{SwiftBridgeModule, SWIFT_BRIDGE_PREFIX};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

impl SwiftBridgeModule {
    /// Generate the tokens for a shared enum.
    pub(super) fn generate_shared_enum_tokens(
        &self,
        shared_enum: &SharedEnum,
    ) -> Option<TokenStream> {
        let enum_name = &shared_enum.name;
        let swift_bridge_path = &self.swift_bridge_path;

        let enum_ffi_name = format!("{}{}", SWIFT_BRIDGE_PREFIX, enum_name);
        let enum_ffi_name = Ident::new(&enum_ffi_name, enum_name.span());

        let mut enum_variants = vec![];
        let mut enum_ffi_variants = vec![];

        for variant in shared_enum.variants.iter() {
            let variant_name = &variant.name;
            let v = quote! {
                #variant_name
            };
            enum_variants.push(v);
        }

        for variant in shared_enum.variants.iter() {
            let variant_name = &variant.name;
            let v = quote! {
                #variant_name
            };
            enum_ffi_variants.push(v);
        }

        let mut convert_rust_variants_to_ffi = vec![];
        let mut convert_ffi_variants_to_rust = vec![];

        for variant in shared_enum.variants.iter() {
            let variant_name = &variant.name;
            let v = quote! {
                #enum_name :: #variant_name => #enum_ffi_name :: #variant_name
            };
            convert_rust_variants_to_ffi.push(v);
        }

        for variant in shared_enum.variants.iter() {
            let variant_name = &variant.name;
            let v = quote! {
                #enum_ffi_name :: #variant_name => #enum_name :: #variant_name
            };
            convert_ffi_variants_to_rust.push(v);
        }

        let definition = quote! {
            pub enum #enum_name {
                #(#enum_variants),*
            }

            #[repr(C)]
            #[doc(hidden)]
            pub enum #enum_ffi_name {
                #(#enum_ffi_variants),*
            }

            impl #swift_bridge_path::SharedEnum for #enum_name {
                type FfiRepr = #enum_ffi_name;
            }

            impl #enum_name {
                #[doc(hidden)]
                #[inline(always)]
                pub fn into_ffi_repr(self) -> #enum_ffi_name {
                    match self {
                        #(#convert_rust_variants_to_ffi),*
                    }
                }
            }

            impl #enum_ffi_name {
                #[doc(hidden)]
                #[inline(always)]
                pub fn into_rust_repr(self) -> #enum_name {
                    match self {
                        #(#convert_ffi_variants_to_rust),*
                    }
                }
            }
        };

        Some(definition)
    }
}
