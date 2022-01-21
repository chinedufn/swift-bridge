use crate::SWIFT_BRIDGE_PREFIX;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::fmt::{Debug, Formatter};
use syn::LitStr;

pub(crate) use self::struct_field::{NamedStructField, StructFields, UnnamedStructField};

mod struct_field;

#[derive(Clone)]
pub(crate) struct SharedStruct {
    pub name: Ident,
    pub swift_repr: StructSwiftRepr,
    pub fields: StructFields,
    pub swift_name: Option<LitStr>,
    pub already_declared: bool,
}

impl SharedStruct {
    pub(crate) fn swift_name_string(&self) -> String {
        match self.swift_name.as_ref() {
            Some(ty) => ty.value(),
            None => self.name.to_string(),
        }
    }

    pub(crate) fn ffi_name_string(&self) -> String {
        let name = self.swift_name_string();

        format!("{}${}", SWIFT_BRIDGE_PREFIX, name)
    }

    pub(crate) fn ffi_name_tokens(&self) -> TokenStream {
        let name = Ident::new(
            &format!("{}{}", SWIFT_BRIDGE_PREFIX, &self.name),
            self.name.span(),
        );

        quote! {
            #name
        }
    }
}

impl SharedStruct {
    /// Convert the FFI representation of this struct into its Rust struct format.
    pub(crate) fn convert_ffi_repr_to_rust(&self, rust_val: &TokenStream) -> TokenStream {
        let struct_name = &self.name;

        let converted_fields = match &self.fields {
            StructFields::Named(named) => {
                let converted_fields = self.convert_named_fields_tokens(named, |field| {
                    let field_name = &field.name;

                    quote! {
                        #field_name: val.#field_name
                    }
                });

                quote! {
                    { #converted_fields }
                }
            }
            StructFields::Unnamed(unnamed) => {
                let converted_fields = self.convert_unnamed_fields_tokens(unnamed, |field| {
                    let field_accessor = field.rust_field_accessor();

                    quote! { val.#field_accessor }
                });

                quote! {
                    ( #converted_fields )
                }
            }
            StructFields::Unit => {
                quote! {}
            }
        };

        if self.fields.is_empty() {
            quote! {
                #struct_name #converted_fields
            }
        } else {
            quote! {
                { let val = #rust_val; #struct_name #converted_fields }
            }
        }
    }

    pub(crate) fn convert_rust_expression_to_ffi_repr(
        &self,
        expression: &TokenStream,
    ) -> TokenStream {
        let converted_fields = match &self.fields {
            StructFields::Named(named) => {
                let converted_fields = self.convert_named_fields_tokens(named, |field| {
                    let field_name = &field.name;

                    quote! {
                        #field_name: val.#field_name
                    }
                });

                quote! {
                    { #converted_fields }
                }
            }
            StructFields::Unnamed(unnamed) => {
                let converted_fields = self.convert_unnamed_fields_tokens(unnamed, |field| {
                    let rust_accessor = field.rust_field_accessor();

                    quote! { val.#rust_accessor }
                });

                quote! {
                    ( #converted_fields )
                }
            }
            StructFields::Unit => {
                quote! {}
            }
        };

        let ffi_name = self.ffi_name_tokens();

        if self.fields.is_empty() {
            quote! {
                #ffi_name { _private: 123 }
            }
        } else {
            quote! {
                { let val = #expression; #ffi_name #converted_fields }
            }
        }
    }

    pub(crate) fn convert_swift_to_ffi_repr(&self, expression: &str) -> String {
        let struct_name = &self.ffi_name_string();

        let converted_fields = match &self.fields {
            StructFields::Named(named) => {
                let converted_fields = self.convert_named_fields_string(named, move |field| {
                    let field_name = &field.name;

                    format!("{field_name}: val.{field_name}", field_name = field_name)
                });
                converted_fields
            }
            StructFields::Unnamed(unnamed) => {
                let converted_fields = self.convert_unnamed_fields_string(unnamed, move |field| {
                    let field_name = &field.ffi_field_name();

                    format!("{field_name}: val.{field_name}", field_name = field_name)
                });
                converted_fields
            }
            StructFields::Unit => "".to_string(),
        };

        if self.fields.is_empty() {
            format!("{struct_name}(_private: 123)", struct_name = &struct_name,)
        } else {
            format!(
                "{{ let val = {expression}; return {struct_name}({converted_fields}); }}()",
                struct_name = &struct_name,
                expression = expression,
                converted_fields = converted_fields
            )
        }
    }

    pub(crate) fn convert_ffi_expression_to_swift(&self, expression: &str) -> String {
        let struct_name = &self.swift_name_string();

        let converted_fields = match &self.fields {
            StructFields::Named(named) => {
                let converted_fields = self.convert_named_fields_string(named, move |field| {
                    let field_name = &field.name;
                    format!("{field_name}: val.{field_name}", field_name = field_name)
                });

                converted_fields
            }
            StructFields::Unnamed(unnamed) => {
                let converted_fields = self.convert_unnamed_fields_string(unnamed, move |field| {
                    let field_name = &field.ffi_field_name();

                    format!("{field_name}: val.{field_name}", field_name = field_name)
                });

                converted_fields
            }
            StructFields::Unit => "".to_string(),
        };

        if self.fields.is_empty() {
            format!("{struct_name}()", struct_name = &struct_name,)
        } else {
            format!(
                "{{ let val = {expression}; return {struct_name}({converted_fields}); }}()",
                expression = expression,
                struct_name = &struct_name,
                converted_fields = converted_fields
            )
        }
    }

    fn convert_named_fields_tokens<F: Fn(&NamedStructField) -> TokenStream>(
        &self,
        named: &[NamedStructField],
        field_converter: F,
    ) -> TokenStream {
        let mut converted_fields = vec![];

        for field in named {
            let field = field_converter(field);

            converted_fields.push(field)
        }

        let converted_fields = quote! {
            #(#converted_fields),*
        };

        converted_fields
    }

    fn convert_named_fields_string<F: Fn(&NamedStructField) -> String>(
        &self,
        named: &[NamedStructField],
        field_converter: F,
    ) -> String {
        let mut converted_fields = vec![];

        for field in named {
            let field = field_converter(field);

            converted_fields.push(field)
        }

        converted_fields.join(", ")
    }

    fn convert_unnamed_fields_tokens<F: Fn(&UnnamedStructField) -> TokenStream>(
        &self,
        unnamed: &[UnnamedStructField],
        field_converter: F,
    ) -> TokenStream {
        let mut converted_fields = vec![];

        for field in unnamed {
            let field = field_converter(field);

            converted_fields.push(field)
        }

        let converted_fields = quote! {
            #(#converted_fields),*
        };

        converted_fields
    }

    fn convert_unnamed_fields_string<F: Fn(&UnnamedStructField) -> String>(
        &self,
        unnamed: &[UnnamedStructField],
        field_converter: F,
    ) -> String {
        let mut converted_fields = vec![];

        for field in unnamed {
            let field = field_converter(field);

            converted_fields.push(field)
        }

        converted_fields.join(", ")
    }
}

impl PartialEq for SharedStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_string() == other.name.to_string()
            && self.swift_repr == other.swift_repr
            && self.fields == other.fields
            && self.swift_name.as_ref().map(|l| l.value())
                == other.swift_name.as_ref().map(|l| l.value())
            && self.already_declared == other.already_declared
    }
}

impl Debug for SharedStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedStruct")
            .field("name", &self.name.to_string())
            .field("swift_repr", &self.swift_repr)
            .field("fields", &self.fields)
            .field("swift_name", &self.swift_name.as_ref().map(|l| l.value()))
            .field("already_declared", &self.already_declared)
            .finish()
    }
}

/// Whether to create a class or a structure when creating the Swift representation of a shared
/// struct.
///
/// https://docs.swift.org/swift-book/LanguageGuide/ClassesAndStructures.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum StructSwiftRepr {
    Class,
    /// # Invariants
    ///
    /// (These invariants aren't implemented yet)
    ///
    /// - Cannot be owned by Swift it it contains one or more fields that need to run destructors.
    ///   - Since Swift struct cannot run de-initializers on structs. Only on classes.
    /// - Can always be passed to Swift by immutable reference
    ///   - Since this means Swift does not need to run any de-initializers, which it cannot do
    ///     for structs.
    Structure,
}
