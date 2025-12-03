//! Tests for the `#[swift_bridge::bridged]` attribute macro.

use crate::bridged_struct::generate_bridged_struct_tokens;

/// Verify that we generate correct tokens for a basic struct with primitive fields.
#[test]
fn bridged_struct_basic() {
    let input: syn::ItemStruct = syn::parse_quote! {
        pub struct BasicStruct {
            pub flag: bool,
            pub count: u32,
        }
    };

    let tokens = generate_bridged_struct_tokens(input);
    let output = tokens.to_string();

    // Verify original struct is preserved
    assert!(output.contains("pub struct BasicStruct"));

    // Verify FFI struct is generated
    assert!(output.contains("__swift_bridge__BasicStruct"));
    // Note: quote! generates "# [repr (C)]" with spaces
    assert!(output.contains("repr (C)"));

    // Verify SharedStruct impl
    assert!(output.contains("impl swift_bridge :: SharedStruct for BasicStruct"));

    // Verify into_ffi_repr method
    assert!(output.contains("fn into_ffi_repr"));

    // Verify into_rust_repr method
    assert!(output.contains("fn into_rust_repr"));

    // Verify Option support
    assert!(output.contains("__swift_bridge__Option_BasicStruct"));

    // Verify Swift constant is generated
    assert!(output.contains("__SWIFT_BRIDGE_BRIDGED_SWIFT_BASICSTRUCT"));

    // Verify C header constant is generated
    assert!(output.contains("__SWIFT_BRIDGE_BRIDGED_HEADER_BASICSTRUCT"));
}

/// Verify that empty structs get a private field to be FFI-safe.
#[test]
fn bridged_struct_empty() {
    let input: syn::ItemStruct = syn::parse_quote! {
        pub struct EmptyStruct {}
    };

    let tokens = generate_bridged_struct_tokens(input);
    let output = tokens.to_string();

    // Empty structs need a private field for FFI safety
    assert!(output.contains("_private : u8"));
}

/// Verify that structs with String fields are handled correctly.
#[test]
fn bridged_struct_with_string() {
    let input: syn::ItemStruct = syn::parse_quote! {
        pub struct WithString {
            pub message: String,
        }
    };

    let tokens = generate_bridged_struct_tokens(input);
    let output = tokens.to_string();

    // String fields should be converted to RustString pointers in FFI
    assert!(output.contains("swift_bridge :: string :: RustString"));
}

/// Verify that struct attributes are preserved.
#[test]
fn bridged_struct_preserves_attributes() {
    let input: syn::ItemStruct = syn::parse_quote! {
        #[derive(Debug, Clone)]
        pub struct WithAttributes {
            pub value: i32,
        }
    };

    let tokens = generate_bridged_struct_tokens(input);
    let output = tokens.to_string();

    // Original derive attributes should be preserved (with quote! spacing)
    assert!(output.contains("derive (Debug , Clone)"));
}

/// Verify Swift code generation for a basic struct.
#[test]
fn bridged_struct_swift_code() {
    let input: syn::ItemStruct = syn::parse_quote! {
        pub struct SwiftTestStruct {
            pub count: u32,
            pub active: bool,
        }
    };

    let tokens = generate_bridged_struct_tokens(input);
    let output = tokens.to_string();

    // The Swift code should be in the constant
    assert!(output.contains("public struct SwiftTestStruct"));
    assert!(output.contains("UInt32")); // u32 -> UInt32 in Swift
    assert!(output.contains("Bool")); // bool -> Bool in Swift
    assert!(output.contains("intoFfiRepr"));
    assert!(output.contains("intoSwiftRepr"));
}

/// Verify C header generation for a basic struct.
#[test]
fn bridged_struct_c_header() {
    let input: syn::ItemStruct = syn::parse_quote! {
        pub struct CTestStruct {
            pub count: u32,
            pub active: bool,
        }
    };

    let tokens = generate_bridged_struct_tokens(input);
    let output = tokens.to_string();

    // The C header should be in the constant
    assert!(output.contains("typedef struct"));
    assert!(output.contains("uint32_t")); // u32 -> uint32_t in C
    assert!(output.contains("bool")); // bool -> bool in C (from stdbool.h)
}

/// Verify that unnamed structs (tuple structs) are rejected.
#[test]
fn bridged_struct_rejects_unnamed() {
    let input: syn::ItemStruct = syn::parse_quote! {
        pub struct TupleStruct(u32, bool);
    };

    let tokens = generate_bridged_struct_tokens(input);
    let output = tokens.to_string();

    // Should contain a compile error
    assert!(output.contains("compile_error"));
    assert!(output.contains("only supports structs with named fields"));
}
