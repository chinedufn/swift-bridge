# Adding support for a signature

Bridge modules expose Rust and Swift functions by declaring their function signatures.

For example, in the following bridge module we declare one Swift and one Rust function signature.

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        async fn add(a: u8, b: u16) -> u32;
    }

    extern "Swift" {
        type Counter;
        fn increment(&mut self);
    }
}
```

Not all signatures are supported. For example, the following would not compile:

```rust
// This does not compile

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        async fn print_cow(cow: Cow<'static, str>);
    }
}
```

`swift-bridge` does not currently support the `Cow<'static, str>`, so the `print_cow` function signature is unsupported.

This chapter shows how to add support for an unsupported signature.

## Implementing Support for a Signature

To support a new signature, we first write automated tests for the signature and then implement just enough code to get those
tests passing.

Add the time of writing, the `Swift` programming language does not have support for 128 bit integers.

Let's pretend that `Swift` gained support for them and we were tasked with supporting `u128` argument and return types
in `swift-bridge` function signatures.

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn reflect_u128(num: u128);
    }
}

fn reflect_u128(num: u128) -> num {
    num
}
```

#### Integration Tests

Our first step would be to add an integration test where we declared this signature in
a bridge module and called the function from Rust.

We would first find a good place in `crates/swift-integration-tests` to declare the signature.

[`crates/swift-integration-tests/src/primitive.rs`](https://github.com/chinedufn/swift-bridge/blob/master/crates/swift-integration-tests/src/primitive.rs)
would be a good choice.

Before adding our `u128` support, the file looks like this:

```rust
{{#include ../../../../crates/swift-integration-tests/src/primitive.rs::10}}
// ... snip ...
```

Next we would add our `reflect_u128` function to the bridge module.

We would then modify the `SwiftRustIntegrationTestRunner` to call our function.

In this case we would want to modify [`SwiftRustIntegrationTestRunner/SwiftRustIntegrationTestRunnerTests/PrimitiveTests.swift`](https://github.com/chinedufn/swift-bridge/blob/master/SwiftRustIntegrationTestRunner/SwiftRustIntegrationTestRunnerTests/PrimitiveTests.swift),
which before our updates looks something like:

```rust
import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests for generic types such as `type SomeType<u32>`
class PrimitiveTests: XCTestCase {
    /// Run tests where Rust calls Swift functions that take primitive args.
    func testRustCallsSwiftPrimitives() throws {
        test_rust_calls_swift_primitives()
    }
    
    /// Run tests where Swift calls Rust functions that take primitive args.
    func testSwiftCallsRustPrimitives() throws {
        XCTAssertEqual(rust_double_u8(10), 20);
        XCTAssertEqual(rust_double_i8(10), 20);
        XCTAssertEqual(rust_double_u16(10), 20);
        XCTAssertEqual(rust_double_i16(10), 20);
        XCTAssertEqual(rust_double_u32(10), 20);
        XCTAssertEqual(rust_double_i32(10), 20);
        XCTAssertEqual(rust_double_u64(10), 20);
        XCTAssertEqual(rust_double_i64(10), 20);
        XCTAssertEqual(rust_double_f32(10.0), 20.0);
        XCTAssertEqual(rust_double_f64(10.0), 20.0);
        XCTAssertEqual(rust_negate_bool(true), false);
        XCTAssertEqual(rust_negate_bool(false), true);
    }
}
```

#### Codegen Tests

After adding one or more integration tests, we would then add one or more codegen tests.

In codegen tests we write out the exact code that we expect `swift-bridge` to generate.

For example, here is the codegen test for supporting `Option<u8>` in Rust function arguments.

```rust
// Copied from: crates/swift-bridge-ir/src/codegen/codegen_tests/option_codegen_tests.rs

/// Test code generation for Rust function that accepts and returns an Option<T> where T is a
/// primitive.
mod extern_rust_fn_option_primitive {
    use super::*;

    fn bridge_module_tokens() -> TokenStream {
        quote! {
            mod ffi {
                extern "Rust" {
                    fn some_function (arg: Option<u8>) -> Option<f32>;
                }
            }
        }
    }

    fn expected_rust_tokens() -> ExpectedRustTokens {
        ExpectedRustTokens::Contains(quote! {
            #[export_name = "__swift_bridge__$some_function"]
            pub extern "C" fn __swift_bridge__some_function(
                arg: swift_bridge::option::OptionU8
            ) -> swift_bridge::option::OptionF32 {
                if let Some(val) = super::some_function(
                    if arg.is_some {
                        Some(arg.val)
                    } else {
                        None
                    }
                ) {
                    swift_bridge::option::OptionF32 { val, is_some: true}
                } else {
                    swift_bridge::option::OptionF32 { val: 123.4, is_some: false}
                }
            }
        })
    }

    fn expected_swift_code() -> ExpectedSwiftCode {
        ExpectedSwiftCode::ContainsAfterTrim(
            r#"
func some_function(_ arg: Optional<UInt8>) -> Optional<Float> {
    { let val = __swift_bridge__$some_function({ let val = arg; return __private__OptionU8(val: val ?? 123, is_some: val != nil); }()); if val.is_some { return val.val } else { return nil } }()
}
"#,
        )
    }

    const EXPECTED_C_HEADER: ExpectedCHeader = ExpectedCHeader::ExactAfterTrim(
        r#"
struct __private__OptionF32 __swift_bridge__$some_function(struct __private__OptionU8 arg);
    "#,
    );

    #[test]
    fn extern_rust_fn_option_primitive() {
        CodegenTest {
            bridge_module: bridge_module_tokens().into(),
            expected_rust_tokens: expected_rust_tokens(),
            expected_swift_code: expected_swift_code(),
            expected_c_header: EXPECTED_C_HEADER,
        }
        .test();
    }
}
```

#### Passing Tests

After writing our integration and codegen tests we would add just enough code to make them pass.

This would involve modifying `crates/swift-bridge-ir/src/bridged_type.rs` until all of our tests passed.










