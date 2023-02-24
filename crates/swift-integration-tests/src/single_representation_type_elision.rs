/// Tests that we can pass types that has exactly one FFI representation to/from Swift.
///
/// We test this explicitly since our codegen elides types that have exactly one representation.
///
/// See crates/swift-bridge-ir/src/codegen/codegen_tests/single_representation_type_elision_codegen_tests.rs
#[swift_bridge::bridge]
mod ffi {
    struct SingleReprTestUnitStruct;

    extern "Rust" {
        fn rust_one_null_arg(arg: ()) -> ();
        fn rust_two_null_args(arg1: (), arg2: ()) -> ();

        fn rust_one_unit_struct(arg: SingleReprTestUnitStruct) -> SingleReprTestUnitStruct;
        fn rust_two_unit_structs(
            arg1: SingleReprTestUnitStruct,
            arg2: SingleReprTestUnitStruct,
        ) -> SingleReprTestUnitStruct;
    }
}
use ffi::SingleReprTestUnitStruct;

fn rust_one_null_arg(_arg: ()) -> () {
    ()
}
fn rust_two_null_args(_arg1: (), _arg2: ()) -> () {
    ()
}

fn rust_one_unit_struct(_arg: SingleReprTestUnitStruct) -> SingleReprTestUnitStruct {
    SingleReprTestUnitStruct
}
fn rust_two_unit_structs(
    _arg1: SingleReprTestUnitStruct,
    _arg2: SingleReprTestUnitStruct,
) -> SingleReprTestUnitStruct {
    SingleReprTestUnitStruct
}
