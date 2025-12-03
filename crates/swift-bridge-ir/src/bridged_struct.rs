//! Implementation for the `#[swift_bridge::bridged]` attribute macro.
//!
//! This macro allows users to define structs outside of bridge modules while still
//! generating all the necessary FFI glue code.
//!
//! This implementation reuses the existing `BridgedType` infrastructure to ensure
//! consistency with bridge module codegen and automatic support for new types.

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Fields, ItemStruct, Type, Visibility};

use crate::bridged_type::{BridgeableType, BridgedType, TypePosition};
use crate::parse::TypeDeclarations;
use crate::SWIFT_BRIDGE_PREFIX;

/// Represents a parsed field from the struct.
struct ParsedField {
    name: Ident,
    ty: Type,
    #[allow(dead_code)]
    vis: Visibility,
}

/// Helper to get the swift_bridge path for codegen.
fn swift_bridge_path() -> syn::Path {
    syn::parse_quote!(swift_bridge)
}

/// Information about a parsed custom type (not recognized by BridgedType).
enum CustomTypeInfo {
    /// Vec<T> where T is a custom type
    Vec { inner_type: String },
    /// Option<Vec<T>> where T is a custom type - uses nullable pointer pattern
    OptionVec { inner_type: String },
    /// Option<T> where T is a custom type (not Vec) - uses is_some struct pattern
    Option { inner_type: String },
    /// A plain custom type (another bridged struct)
    Plain { type_name: String },
}

impl CustomTypeInfo {
    /// Parse a syn::Type that BridgedType couldn't recognize.
    fn from_type(ty: &Type) -> Self {
        let type_str = quote::quote!(#ty).to_string().replace(' ', "");

        if type_str.starts_with("Vec<") && type_str.ends_with('>') {
            let inner = &type_str[4..type_str.len() - 1];
            CustomTypeInfo::Vec {
                inner_type: inner.to_string(),
            }
        } else if type_str.starts_with("Option<Vec<") && type_str.ends_with(">>") {
            // Option<Vec<T>> - extract T from "Option<Vec<T>>"
            let inner = &type_str[11..type_str.len() - 2];
            CustomTypeInfo::OptionVec {
                inner_type: inner.to_string(),
            }
        } else if type_str.starts_with("Option<") && type_str.ends_with('>') {
            let inner = &type_str[7..type_str.len() - 1];
            CustomTypeInfo::Option {
                inner_type: inner.to_string(),
            }
        } else {
            CustomTypeInfo::Plain {
                type_name: type_str,
            }
        }
    }

    /// Get the Swift type for this custom type.
    fn to_swift_type(&self) -> String {
        match self {
            CustomTypeInfo::Vec { inner_type } => format!("[{}]", inner_type),
            CustomTypeInfo::OptionVec { inner_type } => format!("[{}]?", inner_type),
            CustomTypeInfo::Option { inner_type } => format!("{}?", inner_type),
            CustomTypeInfo::Plain { type_name } => type_name.clone(),
        }
    }

    /// Get the FFI Rust type for this custom type.
    fn to_ffi_rust_type(&self) -> TokenStream {
        match self {
            CustomTypeInfo::Vec { inner_type } => {
                let ffi_inner = format_ident!("{}{}", SWIFT_BRIDGE_PREFIX, inner_type);
                quote! { *mut Vec<#ffi_inner> }
            }
            CustomTypeInfo::OptionVec { inner_type } => {
                // Option<Vec<T>> uses nullable pointer pattern
                let ffi_inner = format_ident!("{}{}", SWIFT_BRIDGE_PREFIX, inner_type);
                quote! { *mut Vec<#ffi_inner> }
            }
            CustomTypeInfo::Option { inner_type } => {
                let ffi_option = format_ident!("{}Option_{}", SWIFT_BRIDGE_PREFIX, inner_type);
                quote! { #ffi_option }
            }
            CustomTypeInfo::Plain { type_name } => {
                let ffi_name = format_ident!("{}{}", SWIFT_BRIDGE_PREFIX, type_name);
                quote! { #ffi_name }
            }
        }
    }

    /// Get the C type for this custom type.
    fn to_c_type(&self) -> String {
        match self {
            CustomTypeInfo::Vec { inner_type } => {
                // Vec is passed as a pointer
                format!("void* /* Vec<{}> */", inner_type)
            }
            CustomTypeInfo::OptionVec { inner_type } => {
                // Option<Vec<T>> uses nullable pointer pattern
                format!("void* /* Option<Vec<{}>> */", inner_type)
            }
            CustomTypeInfo::Option { inner_type } => {
                format!("{}$Option${}", SWIFT_BRIDGE_PREFIX, inner_type)
            }
            CustomTypeInfo::Plain { type_name } => {
                format!("{}${}", SWIFT_BRIDGE_PREFIX, type_name)
            }
        }
    }

    /// Generate Rust code to convert from Rust type to FFI type.
    fn rust_to_ffi_conversion(&self, expr: &TokenStream) -> TokenStream {
        match self {
            CustomTypeInfo::Vec { inner_type: _ } => {
                quote! {
                    Box::into_raw(Box::new(
                        #expr.into_iter().map(|v| v.into_ffi_repr()).collect::<Vec<_>>()
                    ))
                }
            }
            CustomTypeInfo::OptionVec { inner_type: _ } => {
                // Option<Vec<T>> uses nullable pointer pattern
                quote! {
                    if let Some(val) = #expr {
                        Box::into_raw(Box::new(
                            val.into_iter().map(|v| v.into_ffi_repr()).collect::<Vec<_>>()
                        ))
                    } else {
                        std::ptr::null_mut()
                    }
                }
            }
            CustomTypeInfo::Option { inner_type } => {
                let ffi_option = format_ident!("{}Option_{}", SWIFT_BRIDGE_PREFIX, inner_type);
                quote! {
                    if let Some(val) = #expr {
                        #ffi_option { is_some: true, val: val.into_ffi_repr() }
                    } else {
                        #ffi_option { is_some: false, val: unsafe { std::mem::zeroed() } }
                    }
                }
            }
            CustomTypeInfo::Plain { type_name: _ } => {
                quote! { #expr.into_ffi_repr() }
            }
        }
    }

    /// Generate Rust code to convert from FFI type to Rust type.
    fn ffi_to_rust_conversion(&self, expr: &TokenStream) -> TokenStream {
        match self {
            CustomTypeInfo::Vec { inner_type } => {
                let inner_ident = format_ident!("{}", inner_type);
                quote! {
                    unsafe { *Box::from_raw(#expr) }
                        .into_iter()
                        .map(|v| v.into_rust_repr())
                        .collect::<Vec<#inner_ident>>()
                }
            }
            CustomTypeInfo::OptionVec { inner_type } => {
                // Option<Vec<T>> uses nullable pointer pattern
                let inner_ident = format_ident!("{}", inner_type);
                quote! {
                    if #expr.is_null() {
                        None
                    } else {
                        Some(
                            unsafe { *Box::from_raw(#expr) }
                                .into_iter()
                                .map(|v| v.into_rust_repr())
                                .collect::<Vec<#inner_ident>>()
                        )
                    }
                }
            }
            CustomTypeInfo::Option { inner_type } => {
                let inner_ident = format_ident!("{}", inner_type);
                quote! {
                    if #expr.is_some {
                        Some(#expr.val.into_rust_repr())
                    } else {
                        None::<#inner_ident>
                    }
                }
            }
            CustomTypeInfo::Plain { type_name: _ } => {
                quote! { #expr.into_rust_repr() }
            }
        }
    }

    /// Generate Swift code to convert from Swift type to FFI type.
    fn swift_to_ffi_conversion(&self, expr: &str) -> String {
        match self {
            CustomTypeInfo::Vec { inner_type } => {
                // Create a Vec on the Rust side by calling a helper
                format!(
                    "{{ let arr = {expr}; var vec = RustVec<__swift_bridge__${inner_type}>(); for item in arr {{ vec.push(value: item.intoFfiRepr()) }}; vec.isOwned = false; return vec.ptr }}()",
                    expr = expr,
                    inner_type = inner_type
                )
            }
            CustomTypeInfo::OptionVec { inner_type } => {
                // Option<Vec<T>> uses nullable pointer pattern
                format!(
                    "{{ if let arr = {expr} {{ var vec = RustVec<__swift_bridge__${inner_type}>(); for item in arr {{ vec.push(value: item.intoFfiRepr()) }}; vec.isOwned = false; return vec.ptr }} else {{ return nil }} }}()",
                    expr = expr,
                    inner_type = inner_type
                )
            }
            CustomTypeInfo::Option { inner_type } => {
                format!(
                    "{{ if let val = {expr} {{ return __swift_bridge__$Option${inner_type}(is_some: true, val: val.intoFfiRepr()) }} else {{ return __swift_bridge__$Option${inner_type}(is_some: false, val: __swift_bridge__${inner_type}()) }} }}()",
                    expr = expr,
                    inner_type = inner_type
                )
            }
            CustomTypeInfo::Plain { type_name: _ } => {
                format!("{}.intoFfiRepr()", expr)
            }
        }
    }

    /// Generate Swift code to convert from FFI type to Swift type.
    fn ffi_to_swift_conversion(&self, expr: &str) -> String {
        match self {
            CustomTypeInfo::Vec { inner_type } => {
                format!(
                    "{{ let vec = RustVec<__swift_bridge__${inner_type}>(ptr: {expr}); var arr: [{inner_type}] = []; for i in 0..<Int(vec.len()) {{ arr.append(vec.get(index: UInt(i))!.intoSwiftRepr()) }}; return arr }}()",
                    expr = expr,
                    inner_type = inner_type
                )
            }
            CustomTypeInfo::OptionVec { inner_type } => {
                // Option<Vec<T>> uses nullable pointer pattern
                format!(
                    "{{ let val = {expr}; if val != nil {{ let vec = RustVec<__swift_bridge__${inner_type}>(ptr: val!); var arr: [{inner_type}] = []; for i in 0..<Int(vec.len()) {{ arr.append(vec.get(index: UInt(i))!.intoSwiftRepr()) }}; return arr }} else {{ return nil }} }}()",
                    expr = expr,
                    inner_type = inner_type
                )
            }
            CustomTypeInfo::Option { inner_type: _ } => {
                format!(
                    "{{ let opt = {expr}; if opt.is_some {{ return opt.val.intoSwiftRepr() }} else {{ return nil }} }}()",
                    expr = expr
                )
            }
            CustomTypeInfo::Plain { type_name: _ } => {
                format!("{}.intoSwiftRepr()", expr)
            }
        }
    }
}

/// Generate tokens for a struct annotated with `#[swift_bridge::bridged]`.
///
/// # Arguments
///
/// * `item` - The parsed struct
/// * `swift_name` - Optional Swift name override. If None, uses the Rust struct name.
pub fn generate_bridged_struct_tokens(
    item: ItemStruct,
    swift_name: Option<String>,
) -> TokenStream {
    let struct_name = &item.ident;
    let struct_name_string = struct_name.to_string();
    let swift_struct_name = swift_name.unwrap_or_else(|| struct_name_string.clone());
    let ffi_struct_name = format_ident!("{}{}", SWIFT_BRIDGE_PREFIX, struct_name);
    let option_ffi_name = format_ident!("{}Option_{}", SWIFT_BRIDGE_PREFIX, struct_name);

    // Parse fields
    let fields = match &item.fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| ParsedField {
                name: f.ident.clone().unwrap(),
                ty: f.ty.clone(),
                vis: f.vis.clone(),
            })
            .collect::<Vec<_>>(),
        Fields::Unnamed(_) => {
            return syn::Error::new(
                item.span(),
                "#[swift_bridge::bridged] only supports structs with named fields",
            )
            .to_compile_error();
        }
        Fields::Unit => vec![],
    };

    // Generate FFI struct fields with type conversions
    let ffi_fields = generate_ffi_fields(&fields);
    let ffi_fields_tokens = if fields.is_empty() {
        quote! { _private: u8 }
    } else {
        ffi_fields
    };

    // Generate into_ffi_repr body
    let into_ffi_body = generate_into_ffi_body(&fields, struct_name);

    // Generate into_rust_repr body
    let into_rust_body = generate_into_rust_body(&fields, struct_name);

    // Generate Swift code as string constant
    // Swift code uses swift_struct_name, but FFI types use struct_name_string
    let swift_code = generate_swift_code(&swift_struct_name, &struct_name_string, &fields);
    let swift_const_name = format_ident!(
        "__SWIFT_BRIDGE_BRIDGED_SWIFT_{}",
        struct_name_string.to_uppercase()
    );

    // Generate C header as string constant
    let c_header = generate_c_header(&struct_name_string, &fields);
    let c_header_const_name = format_ident!(
        "__SWIFT_BRIDGE_BRIDGED_HEADER_{}",
        struct_name_string.to_uppercase()
    );

    // Preserve original struct attributes and visibility
    let attrs = &item.attrs;
    let vis = &item.vis;
    let generics = &item.generics;
    let original_fields = &item.fields;

    let definition = quote! {
        // Original struct (preserved)
        #(#attrs)*
        #vis struct #struct_name #generics #original_fields

        // FFI representation
        #[repr(C)]
        #[doc(hidden)]
        pub struct #ffi_struct_name {
            #ffi_fields_tokens
        }

        // SharedStruct trait implementation
        impl swift_bridge::SharedStruct for #struct_name {
            type FfiRepr = #ffi_struct_name;
        }

        // Conversion from Rust to FFI
        impl #struct_name {
            #[doc(hidden)]
            #[inline(always)]
            pub fn into_ffi_repr(self) -> #ffi_struct_name {
                #into_ffi_body
            }
        }

        // Conversion from FFI to Rust
        impl #ffi_struct_name {
            #[doc(hidden)]
            #[inline(always)]
            pub fn into_rust_repr(self) -> #struct_name {
                #into_rust_body
            }
        }

        // Option FFI representation
        #[repr(C)]
        #[doc(hidden)]
        pub struct #option_ffi_name {
            is_some: bool,
            val: std::mem::MaybeUninit<#ffi_struct_name>,
        }

        impl #option_ffi_name {
            #[doc(hidden)]
            #[inline(always)]
            pub fn into_rust_repr(self) -> Option<#struct_name> {
                if self.is_some {
                    Some(unsafe { self.val.assume_init().into_rust_repr() })
                } else {
                    None
                }
            }

            #[doc(hidden)]
            #[inline(always)]
            pub fn from_rust_repr(val: Option<#struct_name>) -> #option_ffi_name {
                if let Some(val) = val {
                    #option_ffi_name {
                        is_some: true,
                        val: std::mem::MaybeUninit::new(val.into_ffi_repr())
                    }
                } else {
                    #option_ffi_name {
                        is_some: false,
                        val: std::mem::MaybeUninit::uninit()
                    }
                }
            }
        }

        // Swift code as string constant (for build script to collect)
        #[doc(hidden)]
        pub const #swift_const_name: &str = #swift_code;

        // C header as string constant (for build script to collect)
        #[doc(hidden)]
        pub const #c_header_const_name: &str = #c_header;
    };

    definition
}

/// Generate FFI struct fields with appropriate type conversions.
/// Uses BridgedType infrastructure for consistency with bridge module codegen.
fn generate_ffi_fields(fields: &[ParsedField]) -> TokenStream {
    let types = TypeDeclarations::default();
    let swift_bridge_path = swift_bridge_path();

    let field_tokens: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let ffi_ty = if let Some(bridged_type) = BridgedType::new_with_type(&field.ty, &types) {
                bridged_type.to_ffi_compatible_rust_type(&swift_bridge_path, &types)
            } else {
                // Fallback for custom types (other bridged structs, Vec<CustomType>, etc.)
                CustomTypeInfo::from_type(&field.ty).to_ffi_rust_type()
            };
            quote! { #name: #ffi_ty }
        })
        .collect();

    quote! { #(#field_tokens),* }
}

/// Generate the body of into_ffi_repr method.
/// Uses BridgedType infrastructure for consistency with bridge module codegen.
fn generate_into_ffi_body(fields: &[ParsedField], struct_name: &Ident) -> TokenStream {
    let ffi_struct_name = format_ident!("{}{}", SWIFT_BRIDGE_PREFIX, struct_name);

    if fields.is_empty() {
        return quote! { #ffi_struct_name { _private: 123 } };
    }

    let types = TypeDeclarations::default();
    let swift_bridge_path = swift_bridge_path();

    let field_conversions: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let field_access = quote! { self.#name };
            let conversion =
                if let Some(bridged_type) = BridgedType::new_with_type(&field.ty, &types) {
                    bridged_type.convert_rust_expression_to_ffi_type(
                        &field_access,
                        &swift_bridge_path,
                        &types,
                        Span::call_site(),
                    )
                } else {
                    // Fallback for custom types (other bridged structs, Vec<CustomType>, etc.)
                    CustomTypeInfo::from_type(&field.ty).rust_to_ffi_conversion(&field_access)
                };
            quote! { #name: #conversion }
        })
        .collect();

    quote! {
        #ffi_struct_name {
            #(#field_conversions),*
        }
    }
}

/// Generate the body of into_rust_repr method.
/// Uses BridgedType infrastructure for consistency with bridge module codegen.
fn generate_into_rust_body(fields: &[ParsedField], struct_name: &Ident) -> TokenStream {
    if fields.is_empty() {
        return quote! { #struct_name {} };
    }

    let types = TypeDeclarations::default();
    let swift_bridge_path = swift_bridge_path();

    let field_conversions: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let field_access = quote! { self.#name };
            let conversion =
                if let Some(bridged_type) = BridgedType::new_with_type(&field.ty, &types) {
                    bridged_type.convert_ffi_expression_to_rust_type(
                        &field_access,
                        field.ty.span(),
                        &swift_bridge_path,
                        &types,
                    )
                } else {
                    // Fallback for custom types (other bridged structs, Vec<CustomType>, etc.)
                    CustomTypeInfo::from_type(&field.ty).ffi_to_rust_conversion(&field_access)
                };
            quote! { #name: #conversion }
        })
        .collect();

    quote! {
        #struct_name {
            #(#field_conversions),*
        }
    }
}

/// Generate Swift code for the struct.
/// Uses BridgedType infrastructure for consistency with bridge module codegen.
///
/// # Arguments
///
/// * `swift_name` - The name for the Swift struct
/// * `rust_name` - The Rust struct name (used for FFI types)
/// * `fields` - The struct fields
fn generate_swift_code(swift_name: &str, rust_name: &str, fields: &[ParsedField]) -> String {
    // FFI types always use the Rust name
    let ffi_name = format!("{}${}", SWIFT_BRIDGE_PREFIX, rust_name);
    let option_ffi_name = format!("{}$Option${}", SWIFT_BRIDGE_PREFIX, rust_name);

    let types = TypeDeclarations::default();
    let swift_bridge_path = swift_bridge_path();

    // Generate field declarations
    let field_declarations: Vec<String> = fields
        .iter()
        .map(|f| {
            let swift_ty = if let Some(bridged_type) = BridgedType::new_with_type(&f.ty, &types) {
                bridged_type.to_swift_type(
                    TypePosition::SharedStructField,
                    &types,
                    &swift_bridge_path,
                )
            } else {
                // Fallback for custom types (other bridged structs, Vec<CustomType>, etc.)
                CustomTypeInfo::from_type(&f.ty).to_swift_type()
            };
            format!("    public var {}: {}", f.name, swift_ty)
        })
        .collect();
    let fields_str = if field_declarations.is_empty() {
        "".to_string()
    } else {
        format!("\n{}\n", field_declarations.join("\n"))
    };

    // Generate initializer parameters
    let init_params: Vec<String> = fields
        .iter()
        .map(|f| {
            let swift_ty = if let Some(bridged_type) = BridgedType::new_with_type(&f.ty, &types) {
                bridged_type.to_swift_type(
                    TypePosition::SharedStructField,
                    &types,
                    &swift_bridge_path,
                )
            } else {
                // Fallback for custom types (other bridged structs, Vec<CustomType>, etc.)
                CustomTypeInfo::from_type(&f.ty).to_swift_type()
            };
            format!("{}: {}", f.name, swift_ty)
        })
        .collect();
    let init_params_str = init_params.join(", ");

    // Generate initializer body
    let init_body: Vec<String> = fields
        .iter()
        .map(|f| format!("        self.{} = {}", f.name, f.name))
        .collect();
    let init_body_str = if init_body.is_empty() {
        "".to_string()
    } else {
        format!("\n{}\n    ", init_body.join("\n"))
    };

    // Generate intoFfiRepr field conversions
    let ffi_field_conversions: Vec<String> = fields
        .iter()
        .map(|f| {
            let expr = format!("val.{}", f.name);
            let conversion = if let Some(bridged_type) = BridgedType::new_with_type(&f.ty, &types) {
                bridged_type.convert_swift_expression_to_ffi_type(
                    &expr,
                    &types,
                    TypePosition::SharedStructField,
                )
            } else {
                // Fallback for custom types (other bridged structs, Vec<CustomType>, etc.)
                CustomTypeInfo::from_type(&f.ty).swift_to_ffi_conversion(&expr)
            };
            format!("{}: {}", f.name, conversion)
        })
        .collect();
    let ffi_conversion_str = if ffi_field_conversions.is_empty() {
        format!("{}(_private: 123)", ffi_name)
    } else {
        format!(
            "{{ let val = self; return {}({}); }}()",
            ffi_name,
            ffi_field_conversions.join(", ")
        )
    };

    // Generate intoSwiftRepr field conversions
    let swift_field_conversions: Vec<String> = fields
        .iter()
        .map(|f| {
            let expr = format!("val.{}", f.name);
            let conversion = if let Some(bridged_type) = BridgedType::new_with_type(&f.ty, &types) {
                bridged_type.convert_ffi_expression_to_swift_type(
                    &expr,
                    TypePosition::SharedStructField,
                    &types,
                    &swift_bridge_path,
                )
            } else {
                // Fallback for custom types (other bridged structs, Vec<CustomType>, etc.)
                CustomTypeInfo::from_type(&f.ty).ffi_to_swift_conversion(&expr)
            };
            format!("{}: {}", f.name, conversion)
        })
        .collect();
    let swift_conversion_str = if swift_field_conversions.is_empty() {
        format!("{}()", swift_name)
    } else {
        format!(
            "{{ let val = self; return {}({}); }}()",
            swift_name,
            swift_field_conversions.join(", ")
        )
    };

    format!(
        r#"public struct {swift_name} {{{fields}
    public init({init_params}) {{{init_body}}}

    @inline(__always)
    func intoFfiRepr() -> {ffi_name} {{
        {ffi_conversion}
    }}
}}
extension {ffi_name} {{
    @inline(__always)
    func intoSwiftRepr() -> {swift_name} {{
        {swift_conversion}
    }}
}}
extension {option_ffi_name} {{
    @inline(__always)
    func intoSwiftRepr() -> Optional<{swift_name}> {{
        if self.is_some {{
            return self.val.intoSwiftRepr()
        }} else {{
            return nil
        }}
    }}

    @inline(__always)
    static func fromSwiftRepr(_ val: Optional<{swift_name}>) -> {option_ffi_name} {{
        if let v = val {{
            return {option_ffi_name}(is_some: true, val: v.intoFfiRepr())
        }} else {{
            return {option_ffi_name}(is_some: false, val: {ffi_name}())
        }}
    }}
}}"#,
        swift_name = swift_name,
        fields = fields_str,
        init_params = init_params_str,
        init_body = init_body_str,
        ffi_name = ffi_name,
        option_ffi_name = option_ffi_name,
        ffi_conversion = ffi_conversion_str,
        swift_conversion = swift_conversion_str
    )
}

/// Generate C header code for the struct.
/// Uses BridgedType infrastructure for consistency with bridge module codegen.
fn generate_c_header(struct_name: &str, fields: &[ParsedField]) -> String {
    let ffi_name = format!("{}${}", SWIFT_BRIDGE_PREFIX, struct_name);
    let option_ffi_name = format!("{}$Option${}", SWIFT_BRIDGE_PREFIX, struct_name);

    let types = TypeDeclarations::default();

    // Generate field declarations
    let field_declarations: Vec<String> = if fields.is_empty() {
        vec!["uint8_t _private".to_string()]
    } else {
        fields
            .iter()
            .map(|f| {
                let c_ty = if let Some(bridged_type) = BridgedType::new_with_type(&f.ty, &types) {
                    bridged_type.to_c_type(&types)
                } else {
                    // Fallback for custom types (other bridged structs, Vec<CustomType>, etc.)
                    CustomTypeInfo::from_type(&f.ty).to_c_type()
                };
                format!("{} {}", c_ty, f.name)
            })
            .collect()
    };
    let fields_str = field_declarations.join("; ");

    format!(
        r#"typedef struct {ffi_name} {{ {fields}; }} {ffi_name};
typedef struct {option_ffi_name} {{ bool is_some; {ffi_name} val; }} {option_ffi_name};"#,
        ffi_name = ffi_name,
        option_ffi_name = option_ffi_name,
        fields = fields_str
    )
}

/// Generated Swift and C header code for a bridged struct.
/// Used by the build script for automatic detection of `#[swift_bridge::bridged]` structs.
#[derive(Debug, Clone)]
pub struct BridgedStructCodegen {
    /// Generated Swift code for the struct.
    pub swift: String,
    /// Generated C header code for the struct.
    pub c_header: String,
}

/// Generate Swift and C header code from a struct definition.
///
/// This function is used by the build script to automatically detect
/// `#[swift_bridge::bridged]` structs and generate their Swift/C code.
///
/// # Arguments
///
/// * `item` - The parsed struct item
/// * `swift_name` - Optional Swift name override. If None, uses the Rust struct name.
///
/// # Returns
///
/// Returns `Some(BridgedStructCodegen)` if the struct can be processed,
/// or `None` if it's not a valid bridged struct (e.g., tuple struct).
pub fn generate_bridged_swift_and_c(
    item: &ItemStruct,
    swift_name: Option<String>,
) -> Option<BridgedStructCodegen> {
    let rust_name = item.ident.to_string();
    let swift_name = swift_name.unwrap_or_else(|| rust_name.clone());

    // Parse fields
    let fields = match &item.fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| ParsedField {
                name: f.ident.clone().unwrap(),
                ty: f.ty.clone(),
                vis: f.vis.clone(),
            })
            .collect::<Vec<_>>(),
        Fields::Unnamed(_) => return None,
        Fields::Unit => vec![],
    };

    let swift = generate_swift_code(&swift_name, &rust_name, &fields);
    let c_header = generate_c_header(&rust_name, &fields);

    Some(BridgedStructCodegen { swift, c_header })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_struct() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct BasicStruct {
                pub flag: bool,
                pub count: u32,
            }
        };

        let tokens = generate_bridged_struct_tokens(input, None);
        let output = tokens.to_string();

        assert!(output.contains("pub struct BasicStruct"));
        assert!(output.contains("__swift_bridge__BasicStruct"));
        assert!(output.contains("impl swift_bridge :: SharedStruct for BasicStruct"));
    }

    #[test]
    fn test_empty_struct() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct EmptyStruct {}
        };

        let tokens = generate_bridged_struct_tokens(input, None);
        let output = tokens.to_string();

        assert!(output.contains("_private : u8"));
    }

    #[test]
    fn test_option_primitive() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct OptionalPrimitive {
                pub maybe_count: Option<u32>,
            }
        };

        let tokens = generate_bridged_struct_tokens(input, None);
        let output = tokens.to_string();

        // FFI type should use OptionU32 (from swift_bridge::option)
        assert!(output.contains("swift_bridge :: option :: OptionU32"));
        // Conversion should check is_some
        assert!(output.contains("is_some"));
    }

    // TODO: Option<String> in struct fields is not yet supported by BridgedType infrastructure.
    // See: crates/swift-bridge-ir/src/bridged_type/bridgeable_string.rs:203
    // This test is ignored until upstream support is added.
    #[test]
    #[ignore = "Option<String> in struct fields not yet supported by BridgedType"]
    fn test_option_string() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct OptionalString {
                pub maybe_name: Option<String>,
            }
        };

        let tokens = generate_bridged_struct_tokens(input, None);
        let output = tokens.to_string();

        // FFI type should be nullable pointer
        assert!(output.contains("* mut swift_bridge :: string :: RustString"));
        // Conversion should check for null
        assert!(output.contains("is_null"));
    }

    #[test]
    fn test_swift_code_generation_option_primitive() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct TestOption {
                pub value: Option<i32>,
            }
        };

        let codegen = generate_bridged_swift_and_c(&input, None).unwrap();

        // Swift type should be Optional<Int32>
        assert!(codegen.swift.contains("Optional<Int32>"));
    }

    #[test]
    fn test_c_header_generation_option_primitive() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct TestOption {
                pub value: Option<u64>,
            }
        };

        let codegen = generate_bridged_swift_and_c(&input, None).unwrap();

        // C type should be struct __private__OptionU64
        assert!(codegen.c_header.contains("__private__OptionU64"));
    }

    #[test]
    fn test_vec_primitive() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct VecPrimitive {
                pub values: Vec<u32>,
            }
        };

        let tokens = generate_bridged_struct_tokens(input, None);
        let output = tokens.to_string();

        // FFI type should be *mut Vec<u32>
        assert!(output.contains("* mut Vec < u32 >"));
        // Conversion should use Box::into_raw
        assert!(output.contains("Box :: into_raw"));
        // Conversion should use Box::from_raw
        assert!(output.contains("Box :: from_raw"));
    }

    #[test]
    fn test_swift_code_generation_vec_primitive() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct TestVec {
                pub numbers: Vec<i32>,
            }
        };

        let codegen = generate_bridged_swift_and_c(&input, None).unwrap();

        // Swift type should be RustVec<Int32>
        assert!(codegen.swift.contains("RustVec<Int32>"));
        // Conversion should use RustVec(ptr:) - Swift infers the generic type from context
        assert!(codegen.swift.contains("RustVec(ptr:"));
    }

    #[test]
    fn test_vec_custom_type() {
        // Test Vec<CustomType> where CustomType is another bridged struct
        let input: ItemStruct = syn::parse_quote! {
            pub struct Container {
                pub items: Vec<Product>,
            }
        };

        let tokens = generate_bridged_struct_tokens(input, None);
        let output = tokens.to_string();

        // FFI type should be *mut Vec<__swift_bridge__Product>
        assert!(output.contains("* mut Vec < __swift_bridge__Product >"));
        // Conversion should iterate and call into_ffi_repr
        assert!(output.contains("into_ffi_repr"));
        assert!(output.contains("into_rust_repr"));
    }

    #[test]
    fn test_swift_code_generation_vec_custom_type() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct Container {
                pub items: Vec<Product>,
            }
        };

        let codegen = generate_bridged_swift_and_c(&input, None).unwrap();

        // Swift type should be [Product] (Swift array)
        assert!(codegen.swift.contains("[Product]"));
        // Conversion should use intoFfiRepr/intoSwiftRepr
        assert!(codegen.swift.contains("intoFfiRepr"));
        assert!(codegen.swift.contains("intoSwiftRepr"));
    }

    #[test]
    fn test_option_custom_type() {
        // Test Option<CustomType> where CustomType is another bridged struct
        let input: ItemStruct = syn::parse_quote! {
            pub struct MaybeProduct {
                pub product: Option<Product>,
            }
        };

        let tokens = generate_bridged_struct_tokens(input, None);
        let output = tokens.to_string();

        // FFI type should be __swift_bridge__Option_Product
        assert!(output.contains("__swift_bridge__Option_Product"));
        // Conversion should check is_some
        assert!(output.contains("is_some"));
    }

    #[test]
    fn test_swift_code_generation_option_custom_type() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct MaybeProduct {
                pub product: Option<Product>,
            }
        };

        let codegen = generate_bridged_swift_and_c(&input, None).unwrap();

        // Swift type should be Product? (optional)
        assert!(codegen.swift.contains("Product?"));
    }

    #[test]
    fn test_option_vec_custom_type() {
        // Test Option<Vec<CustomType>> where CustomType is another bridged struct
        let input: ItemStruct = syn::parse_quote! {
            pub struct MaybeProducts {
                pub products: Option<Vec<Product>>,
            }
        };

        let tokens = generate_bridged_struct_tokens(input, None);
        let output = tokens.to_string();

        // FFI type should be *mut Vec<__swift_bridge__Product> (nullable pointer)
        assert!(output.contains("* mut Vec < __swift_bridge__Product >"));
        // Conversion should use null_mut for None
        assert!(output.contains("null_mut"));
        // Conversion should check is_null
        assert!(output.contains("is_null"));
    }

    #[test]
    fn test_swift_code_generation_option_vec_custom_type() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct MaybeProducts {
                pub products: Option<Vec<Product>>,
            }
        };

        let codegen = generate_bridged_swift_and_c(&input, None).unwrap();

        // Swift type should be [Product]? (optional array)
        assert!(codegen.swift.contains("[Product]?"));
        // C type should be void* (nullable pointer)
        assert!(codegen.c_header.contains("void*"));
    }

    #[test]
    fn test_swift_name_attribute() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct RustProduct {
                pub id: u32,
            }
        };

        // With swift_name override
        let codegen =
            generate_bridged_swift_and_c(&input, Some("SwiftProduct".to_string())).unwrap();

        // Swift struct should use the override name
        assert!(codegen.swift.contains("public struct SwiftProduct"));
        // Swift intoSwiftRepr should return the override name
        assert!(codegen.swift.contains("-> SwiftProduct"));
        // FFI types should still use the Rust name
        assert!(codegen.swift.contains("__swift_bridge__$RustProduct"));
        assert!(codegen.c_header.contains("__swift_bridge__$RustProduct"));
    }

    #[test]
    fn test_swift_name_attribute_tokens() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct RustProduct {
                pub id: u32,
            }
        };

        // With swift_name override
        let tokens = generate_bridged_struct_tokens(input, Some("SwiftProduct".to_string()));
        let output = tokens.to_string();

        // Rust struct should keep the original name
        assert!(output.contains("pub struct RustProduct"));
        // FFI struct should use the Rust name
        assert!(output.contains("__swift_bridge__RustProduct"));
        // Swift code constant should use the Swift name
        assert!(output.contains("SwiftProduct"));
    }

    #[test]
    fn test_swift_name_none_uses_rust_name() {
        let input: ItemStruct = syn::parse_quote! {
            pub struct Product {
                pub id: u32,
            }
        };

        // Without swift_name override
        let codegen = generate_bridged_swift_and_c(&input, None).unwrap();

        // Swift struct should use the Rust name
        assert!(codegen.swift.contains("public struct Product"));
        // FFI types should also use the Rust name
        assert!(codegen.swift.contains("__swift_bridge__$Product"));
    }
}
