//! More tests can be found in
//! crates/swift-bridge-ir/src/codegen/codegen_tests/shared_enum_codegen_tests.rs

use crate::bridged_type::SharedEnum;
use crate::codegen::generate_rust_tokens::vec::vec_of_transparent_enum::generate_vec_of_transparent_enum_functions;
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
        if shared_enum.already_declared {
            return None;
        }

        let enum_name = &shared_enum.name;
        let swift_bridge_path = &self.swift_bridge_path;

        let enum_ffi_name = format!("{}{}", SWIFT_BRIDGE_PREFIX, enum_name);
        let enum_ffi_name = Ident::new(&enum_ffi_name, enum_name.span());

        let option_enum = shared_enum.ffi_option_name_tokens();

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

        // TODO:
        //  Parse any derives that the user has specified and combine those with our auto derives.
        let automatic_derives = if shared_enum.has_one_or_more_variants_with_data() {
            vec![]
        } else {
            vec![quote! {Copy}, quote! {Clone}]
        };

        let vec_support = if shared_enum.has_one_or_more_variants_with_data() {
            // Enums with variants that contain data are not yet supported.
            quote! {}
        } else {
            generate_vec_of_transparent_enum_functions(&shared_enum)
        };

        let definition = quote! {
            #[derive(#(#automatic_derives),*)]
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

            #[repr(C)]
            #[doc(hidden)]
            pub struct #option_enum {
                is_some: bool,
                val: std::mem::MaybeUninit<#enum_ffi_name>,
            }

            impl #option_enum {
                #[doc(hidden)]
                #[inline(always)]
                pub fn into_rust_repr(self) -> Option<#enum_name> {
                    if self.is_some {
                        Some(unsafe { self.val.assume_init().into_rust_repr() })
                    } else {
                        None
                    }
                }

                #[doc(hidden)]
                #[inline(always)]
                pub fn from_rust_repr(val: Option<#enum_name>) -> #option_enum {
                    if let Some(val) = val {
                        #option_enum {
                            is_some: true,
                            val: std::mem::MaybeUninit::new(val.into_ffi_repr())
                        }
                    } else {
                        #option_enum {
                            is_some: false,
                            val: std::mem::MaybeUninit::uninit()
                        }
                    }
                }
            }

            #vec_support
        };

        Some(definition)
    }
}
