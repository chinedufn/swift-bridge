#[repr(C)]
#[doc(hidden)]
// Bridges `Result<T, E>` where `T` and `E` are both able to be encoded
// as a pointer.
//
// For example, suppose we have the following bridge module:
//
// ```
// #[swift_bridge::bridge]
// mod ffi {
//     extern "Rust" {
//         type TypeA;
//         type TypeB;
//
//         fn x() -> Result<TypeA, TypeB>;
//     }
//
//     // ---------------------------------
//
//     enum TypeC {
//         Variant1,
//         Variant2
//     }
//     struct TypeD {
//         field: u8
//     }
//
//     extern "Rust" {
//         fn y() -> Result<TypeC, TypeD>
//     }
// }
// ```
//
// When can encode `Ok(TypeA)`, as `ResultPtrAndPtr { is_ok: true, ok_or_err: *mut TypeA as _ }`.
// And we can encode `Err(TypeB)` as `ResultPtrAndPtr { is_ok: false, ok_or_err: *mut TypeB as _ }`.
//
// However, `Ok(TypeC)` and `Err(TypeD)` do not get encoded using `ResultPtrAndPtr` since we do not
// pass transparent enums or transparent structs across the FFI boundary using pointers.
//
// Instead, we would an FFI representation for `Result<TypeC, TypeD>` at compile time.
// See `crates/swift-bridge-ir/src/codegen/codegen_tests/result_codegen_tests.rs` for examples of
// how we generate custom `Result` FFI representations at compile time.
pub struct ResultPtrAndPtr {
    pub is_ok: bool,
    pub ok_or_err: *mut std::ffi::c_void,
}
