//! More tests can be found in
//! crates/swift-bridge-ir/src/codegen/codegen_tests/shared_struct_codegen_tests.rs

use crate::bridged_type::{SharedStruct, StructFields};
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

        let struct_fields = match &shared_struct.fields {
            StructFields::Named(named) => {
                let mut fields = vec![];
                for f in named {
                    let ty = &f.ty;

                    let name = &f.name;
                    let field = quote! {
                        pub #name: #ty
                    };

                    fields.push(field);
                }

                quote! { { #(#fields),* } }
            }
            StructFields::Unnamed(unnamed) => {
                let mut fields = vec![];
                for f in unnamed {
                    let ty = &f.ty;

                    let field = quote! {
                        pub #ty
                    };

                    fields.push(field);
                }

                quote! { ( #(#fields),*  ); }
            }
            StructFields::Unit => {
                quote! {;}
            }
        };

        let repr_c_struct_fields = match &shared_struct.fields {
            StructFields::Named(named) => {
                let mut fields = vec![];
                for f in named {
                    let ty = &f.ty;

                    let name = &f.name;
                    let field = quote! {
                        #name: #ty
                    };

                    fields.push(field);
                }

                quote! { { #(#fields),* } }
            }
            StructFields::Unnamed(unnamed) => {
                let mut fields = vec![];
                for f in unnamed {
                    let ty = &f.ty;

                    let field = quote! {
                        #ty
                    };

                    fields.push(field);
                }

                quote! { ( #(#fields),*  ); }
            }
            StructFields::Unit => {
                quote! {;}
            }
        };

        let convert_rust_to_ffi =
            shared_struct.convert_rust_expression_to_ffi_repr(&quote! { self });
        let convert_ffi_to_rust = shared_struct.convert_ffi_repr_to_rust(&quote! { self });

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
