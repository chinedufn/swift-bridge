// Integration tests for the #[swift_bridge::bridged] attribute macro.

#[swift_bridge::bridged]
pub struct BridgedResponse {
    pub success: bool,
    pub count: u32,
}

#[swift_bridge::bridged]
pub struct BridgedMessage {
    pub text: String,
    pub code: i32,
}

/// Test struct with optional primitive field
#[swift_bridge::bridged]
pub struct BridgedOptionalPrimitive {
    pub value: Option<i32>,
}

// NOTE: Option<String> in struct fields is not yet supported by BridgedType infrastructure.
// See: crates/swift-bridge-ir/src/bridged_type/bridgeable_string.rs:203
// Uncomment when upstream support is added:
// #[swift_bridge::bridged]
// pub struct BridgedOptionalString {
//     pub name: Option<String>,
// }

/// Test struct with Vec<primitive> field
#[swift_bridge::bridged]
pub struct BridgedVecPrimitive {
    pub values: Vec<u32>,
}

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct BridgedResponse;

    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct BridgedMessage;

    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct BridgedOptionalPrimitive;

    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct BridgedVecPrimitive;

    enum BridgedError {
        InvalidInput(String),
        NotFound,
    }

    extern "Rust" {
        fn rust_create_bridged_response(success: bool, count: u32) -> BridgedResponse;
        fn rust_receive_bridged_response(response: BridgedResponse) -> bool;
        fn rust_create_bridged_message(text: String, code: i32) -> BridgedMessage;
        fn rust_fallible_bridged_response(succeed: bool) -> Result<BridgedResponse, BridgedError>;
        fn test_rust_calls_swift_bridged_response();
        fn test_rust_calls_swift_fallible_bridged();

        // Optional field tests
        fn rust_create_optional_primitive_some(value: i32) -> BridgedOptionalPrimitive;
        fn rust_create_optional_primitive_none() -> BridgedOptionalPrimitive;
        fn rust_receive_optional_primitive(opt: BridgedOptionalPrimitive) -> i32;
        // BridgedOptionalString functions removed - Option<String> not yet supported

        // Vec<primitive> field tests
        fn rust_create_vec_primitive(a: u32, b: u32, c: u32) -> BridgedVecPrimitive;
        fn rust_receive_vec_primitive(vec_struct: BridgedVecPrimitive) -> u32;
    }

    extern "Swift" {
        fn swift_create_bridged_response(success: bool, count: u32) -> BridgedResponse;
        fn swift_fallible_bridged_response(succeed: bool) -> Result<BridgedResponse, BridgedError>;
    }
}

fn rust_create_bridged_response(success: bool, count: u32) -> BridgedResponse {
    BridgedResponse { success, count }
}

fn rust_receive_bridged_response(response: BridgedResponse) -> bool {
    response.success && response.count > 0
}

fn rust_create_bridged_message(text: String, code: i32) -> BridgedMessage {
    BridgedMessage { text, code }
}

fn rust_fallible_bridged_response(succeed: bool) -> Result<BridgedResponse, ffi::BridgedError> {
    if succeed {
        Ok(BridgedResponse {
            success: true,
            count: 42,
        })
    } else {
        Err(ffi::BridgedError::InvalidInput("test error".to_string()))
    }
}

fn test_rust_calls_swift_bridged_response() {
    let response = ffi::swift_create_bridged_response(true, 100);
    assert!(response.success);
    assert_eq!(response.count, 100);
}

fn test_rust_calls_swift_fallible_bridged() {
    // Test success case
    match ffi::swift_fallible_bridged_response(true) {
        Ok(response) => {
            assert!(response.success);
            assert_eq!(response.count, 99);
        }
        Err(_) => panic!("Expected Ok, got Err"),
    }

    // Test error case
    match ffi::swift_fallible_bridged_response(false) {
        Ok(_) => panic!("Expected Err, got Ok"),
        Err(ffi::BridgedError::InvalidInput(msg)) => {
            assert_eq!(msg, "Swift error");
        }
        Err(_) => panic!("Wrong error variant"),
    }
}

// Optional primitive field tests
fn rust_create_optional_primitive_some(value: i32) -> BridgedOptionalPrimitive {
    BridgedOptionalPrimitive { value: Some(value) }
}

fn rust_create_optional_primitive_none() -> BridgedOptionalPrimitive {
    BridgedOptionalPrimitive { value: None }
}

fn rust_receive_optional_primitive(opt: BridgedOptionalPrimitive) -> i32 {
    opt.value.unwrap_or(-1)
}

// Vec<primitive> field tests
fn rust_create_vec_primitive(a: u32, b: u32, c: u32) -> BridgedVecPrimitive {
    BridgedVecPrimitive {
        values: vec![a, b, c],
    }
}

fn rust_receive_vec_primitive(vec_struct: BridgedVecPrimitive) -> u32 {
    vec_struct.values.iter().sum()
}
