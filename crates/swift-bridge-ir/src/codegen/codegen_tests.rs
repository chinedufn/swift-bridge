//! Tests for codegen for different scenarios.
//!
//! Tests are grouped into modules, each of which takes a set of tokens and then tests that the
//! generated Rust, Swift and C code matches what we expect.
//!
//! This entire module is conditionally compiled with `#[cfg(test)]`, so there is no need to
//! conditionally compile it's submodules.
//!
//! We previously kept out Rust, Swift and C codegen tests in separate files, but then moved to
//! this approach to make it easier to reason about our codegen.
//!
//! There are a bunch of tests in generate_rust_tokens.rs generate_swift.rs and
//! generate_c_header.rs that were written before this module was created. They should be
//! re-organized into this module over time.

#![cfg(test)]

use crate::test_utils::{
    assert_generated_contains_expected, assert_generated_equals_expected, assert_tokens_contain,
    assert_tokens_eq, parse_ok,
};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

/// Test code generation for freestanding Swift function that takes an opaque Rust type argument.
mod extern_swift_freestanding_fn_with_owned_opaque_rust_type_arg {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    type MyType;
                }

                extern "Swift" {
                    fn some_function (arg: MyType);
                }
            }
        }
    }

    fn expected_rust_tokens() -> TokenStream {
        quote! {
            pub fn some_function (arg: super::MyType) {
                unsafe { __swift_bridge__some_function( Box::into_raw(Box::new(arg)) ) }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$some_function"]
                fn __swift_bridge__some_function (arg: *mut super::MyType);
            }
        }
    }

    const EXPECTED_SWIFT: &'static str = r#"
@_cdecl("__swift_bridge__$some_function")
func __swift_bridge__some_function (_ arg: UnsafeMutableRawPointer) {
    some_function(arg: MyType(ptr: arg, isOwned: true))
}
"#;

    const EXPECTED_C_HEADER: &'static str = r#"
typedef struct MyType MyType;
void __swift_bridge__$MyType$_free(void* self);
"#;

    #[test]
    fn extern_swift_freestanding_fn_with_owned_opaque_rust_type_arg() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: ExpectedRustTokens::Contains(expected_rust_tokens()),
            expected_swift_code: ExpectedSwiftCode::Contains(EXPECTED_SWIFT),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for freestanding Swift function that takes an opaque Swift type argument.
mod extern_swift_freestanding_fn_with_owned_opaque_swift_type_arg {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Swift" {
                    type MyType;
                    fn some_function (arg: MyType);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            pub fn some_function (arg: MyType) {
                unsafe { __swift_bridge__some_function (arg) }
            }

            #[repr(C)]
            pub struct MyType(*mut std::ffi::c_void);

            impl Drop for MyType {
                fn drop (&mut self) {
                    unsafe { __swift_bridge__MyType__free(self.0) }
                }
            }

            extern "C" {
                #[link_name = "__swift_bridge__$some_function"]
                fn __swift_bridge__some_function (arg: MyType);

                #[link_name = "__swift_bridge__$MyType$_free"]
                fn __swift_bridge__MyType__free (this: *mut std::ffi::c_void);
            }
        })
    }

    const EXPECTED_SWIFT_CODE: ExpectedSwiftCode = ExpectedSwiftCode::Contains(
        r#"
@_cdecl("__swift_bridge__$some_function")
func __swift_bridge__some_function (_ arg: __private__PointerToSwiftType) {
    some_function(arg: Unmanaged<MyType>.fromOpaque(arg.ptr).takeRetainedValue())
}
"#,
    );

    const EXPECTED_C_HEADER: &'static str = r#""#;

    #[test]
    fn extern_swift_freestanding_fn_with_owned_opaque_swift_type_arg() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT_CODE,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

/// Test code generation for Rust function that takes an owned String argument.
mod extern_rust_fn_with_owned_string_argument {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod foo {
                extern "Rust" {
                    fn some_function (arg: String);
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: *mut swift_bridge::string::RustString
            ) {
                super::some_function(unsafe { Box::from_raw(arg).0 })
            }
        })
    }

    const EXPECTED_SWIFT_CODE: ExpectedSwiftCode = ExpectedSwiftCode::Contains(
        r#"
func some_function(_ arg: RustString) {
    __swift_bridge__$some_function({arg.isOwned = false; return arg.ptr}())
}
"#,
    );

    const EXPECTED_C_HEADER: &'static str = r#"
void __swift_bridge__$some_function(void* arg);
    "#;

    #[test]
    fn extern_rust_fn_with_owned_string_argument() {
        CodegenTest {
            bridge_module_tokens: bridge_module_tokens(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: EXPECTED_SWIFT_CODE,
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}

struct CodegenTest {
    bridge_module_tokens: TokenStream,
    // Gets turned into a Vec<String> and compared to a Vec<String> of the generated Rust tokens.
    expected_rust_tokens: ExpectedRustTokens,
    // Gets trimmed and compared to the generated Swift code.
    expected_swift_code: ExpectedSwiftCode,
    // Gets trimmed and compared to the generated C header.
    expected_c_header: &'static str,
}

enum ExpectedRustTokens {
    /// The generated Rust token stream matches the provided stream.
    Exact(TokenStream),
    /// The generated Rust tokens stream contains the provided stream.
    Contains(TokenStream),
}

enum ExpectedSwiftCode {
    /// Assert that after we trim our
    ExactAfterTrim(&'static str),
    Contains(&'static str),
}

impl CodegenTest {
    fn test(self) {
        let module = parse_ok(self.bridge_module_tokens);
        let generated_tokens = module.to_token_stream();

        match self.expected_rust_tokens {
            ExpectedRustTokens::Exact(expected_tokens) => {
                assert_tokens_eq(&generated_tokens, &expected_tokens);
            }
            ExpectedRustTokens::Contains(expected_contained_tokens) => {
                assert_tokens_contain(&generated_tokens, &expected_contained_tokens);
            }
        };

        match self.expected_swift_code {
            ExpectedSwiftCode::ExactAfterTrim(expected_swift) => {
                assert_generated_equals_expected(&module.generate_swift(), expected_swift);
            }
            ExpectedSwiftCode::Contains(expected_contained_swift) => {
                assert_generated_contains_expected(
                    &module.generate_swift(),
                    expected_contained_swift,
                );
            }
        };

        assert_generated_equals_expected(&module.generate_c_header_inner(), self.expected_c_header);
    }
}
