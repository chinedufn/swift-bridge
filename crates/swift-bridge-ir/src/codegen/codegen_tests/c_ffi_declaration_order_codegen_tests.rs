use super::{CodegenTest, ExpectedCHeader, ExpectedRustTokens, ExpectedSwiftCode};
use proc_macro2::TokenStream;
use quote::quote;

/// Verify that fields of a structure in a C header file are generated before the structure.
mod structure_fields_generated_before_structure {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    type ResultTestOpaqueRustType;
                }

                extern "Rust" {
                    fn rust_func_return_result_tuple_transparent_enum(
                        succeed: bool,
                    ) -> Result<(i32, ResultTestOpaqueRustType, String), i32>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {})
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(r#""#)
    }

    fn expected_c_header() -> ExpectedCHeader {
        ExpectedCHeader::ContainsAfterTrim(
            r#"
struct __swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32 __swift_bridge__$rust_func_return_result_tuple_transparent_enum(bool succeed);
typedef struct __swift_bridge__$tuple$I32ResultTestOpaqueRustTypeString { int32_t _0; void* _1; void* _2; } __swift_bridge__$tuple$I32ResultTestOpaqueRustTypeString;
typedef enum __swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32$Tag {__swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32$ResultOk, __swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32$ResultErr} __swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32$Tag;
union __swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32$Fields {struct __swift_bridge__$tuple$I32ResultTestOpaqueRustTypeString ok; int32_t err;};
typedef struct __swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32{__swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32$Tag tag; union __swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32$Fields payload;} __swift_bridge__$ResultTupleI32ResultTestOpaqueRustTypeStringAndI32;      
"#,
        )
    }

    #[test]
    fn structure_fields_generated_before_structure() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: expected_c_header(),
        }
        .test();
    }
}
