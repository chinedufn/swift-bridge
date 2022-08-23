#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn test_rust_calls_swift_fn_reflects_owned_opaque_swift_type();
        fn test_rust_calls_swift_method_reflects_owned_opaque_swift_type();
    }

    extern "Swift" {
        type ASwiftType;

        #[swift_bridge(init)]
        fn new(amount: u32) -> ASwiftType;
        fn amount(&self) -> u32;

        fn call_swift_fn_reflects_owned_opaque_swift_type(arg: ASwiftType) -> ASwiftType;
        fn call_swift_method_reflects_owned_opaque_swift_type(&self, arg: ASwiftType)
            -> ASwiftType;
    }
}

fn test_rust_calls_swift_fn_reflects_owned_opaque_swift_type() {
    let swift_ty = ffi::ASwiftType::new(123);
    assert_eq!(swift_ty.amount(), 123);

    let reflected_via_fn = ffi::call_swift_fn_reflects_owned_opaque_swift_type(swift_ty);
    assert_eq!(reflected_via_fn.amount(), 123);
}

fn test_rust_calls_swift_method_reflects_owned_opaque_swift_type() {
    let swift_ty = ffi::ASwiftType::new(123);
    let swift_ty2 = ffi::ASwiftType::new(333);
    assert_eq!(swift_ty2.amount(), 333);

    let reflected_via_method =
        swift_ty.call_swift_method_reflects_owned_opaque_swift_type(swift_ty2);
    assert_eq!(reflected_via_method.amount(), 333);
}
