use crate::generate_core::boxed_fn_support::{
    C_CALLBACK_SUPPORT_NO_ARGS_NO_RETURN, SWIFT_CALLBACK_SUPPORT_NO_ARGS_NO_RETURN,
};
use crate::generate_core::result_support::{C_RESULT_SUPPORT, SWIFT_RUST_RESULT};
use std::path::{Path, PathBuf};

const RUST_STRING_SWIFT: &'static str = include_str!("./generate_core/rust_string.swift");
const RUST_STRING_C: &'static str = include_str!("./generate_core/rust_string.c.h");

const STRING_SWIFT: &'static str = include_str!("./generate_core/string.swift");
const RUST_VEC_SWIFT: &'static str = include_str!("./generate_core/rust_vec.swift");

mod boxed_fn_support;
mod result_support;

pub(super) fn write_core_swift_and_c(out_dir: &Path) {
    let core_swift_out = out_dir.join("SwiftBridgeCore.swift");
    let mut swift = core_swift();
    swift += "\n";
    swift += &RUST_STRING_SWIFT;
    swift += "\n";
    swift += &SWIFT_CALLBACK_SUPPORT_NO_ARGS_NO_RETURN;
    swift += "\n";
    swift += &SWIFT_RUST_RESULT;

    std::fs::write(core_swift_out, swift).unwrap();

    let core_c_header_out = out_dir.join("SwiftBridgeCore.h");
    let mut c_header = core_c_header().to_string();
    c_header += "\n";
    c_header += &RUST_STRING_C;
    c_header += "\n";
    c_header += &C_CALLBACK_SUPPORT_NO_ARGS_NO_RETURN;
    c_header += "\n";
    c_header += &C_RESULT_SUPPORT;

    std::fs::write(core_c_header_out, c_header).unwrap();
}

fn core_swift() -> String {
    let mut core_swift = "".to_string();

    core_swift += STRING_SWIFT;
    core_swift += RUST_VEC_SWIFT;

    for path in vec![
        "src/std_bridge/string.swift",
        "src/std_bridge/rust_vec.swift",
    ] {
        println!(
            "cargo:rerun-if-changed={}",
            PathBuf::from(path).to_str().unwrap()
        )
    }

    for (swift_ty, rust_ty) in vec![
        ("UInt8", "u8"),
        ("UInt16", "u16"),
        ("UInt32", "u32"),
        ("UInt64", "u64"),
        ("UInt", "usize"),
        //
        ("Int8", "i8"),
        ("Int16", "i16"),
        ("Int32", "i32"),
        ("Int64", "i64"),
        ("Int", "isize"),
        //
        ("Bool", "bool"),
    ] {
        core_swift += &conform_to_vectorizable(swift_ty, rust_ty);
    }

    core_swift += &generic_freer();
    core_swift += &generic_copy_type_ffi_repr();

    core_swift
}

fn core_c_header() -> String {
    let mut header = r#"#include <stdint.h>
#include <stdbool.h> 
typedef struct RustStr { uint8_t* const start; uintptr_t len; } RustStr;
typedef struct __private__FfiSlice { void* const start; uintptr_t len; } __private__FfiSlice;
typedef struct __private__PointerToSwiftType { void* ptr; } __private__RustHandleToSwiftType;
void* __swift_bridge__null_pointer(void);

typedef struct __private__OptionU8 { uint8_t val; bool is_some; } __private__OptionU8;
typedef struct __private__OptionI8 { int8_t val; bool is_some; } __private__OptionI8;
typedef struct __private__OptionU16 { uint16_t val; bool is_some; } __private__OptionU16;
typedef struct __private__OptionI16 { int16_t val; bool is_some; } __private__OptionI16;
typedef struct __private__OptionU32 { uint32_t val; bool is_some; } __private__OptionU32;
typedef struct __private__OptionI32 { int32_t val; bool is_some; } __private__OptionI32;
typedef struct __private__OptionU64 { uint64_t val; bool is_some; } __private__OptionU64;
typedef struct __private__OptionI64 { int64_t val; bool is_some; } __private__OptionI64;
typedef struct __private__OptionUsize { uintptr_t val; bool is_some; } __private__OptionUsize;
typedef struct __private__OptionIsize { intptr_t val; bool is_some; } __private__OptionIsize;
typedef struct __private__OptionF32 { float val; bool is_some; } __private__OptionF32;
typedef struct __private__OptionF64 { double val; bool is_some; } __private__OptionDouble;
typedef struct __private__OptionBool { bool val; bool is_some; } __private__OptionBool;
"#
    .to_string();

    for (rust_ty, c_ty) in vec![
        ("u8", "uint8_t"),
        ("u16", "uint16_t"),
        ("u32", "uint32_t"),
        ("u64", "uint64_t"),
        ("usize", "uintptr_t"),
        //
        ("i8", "int8_t"),
        ("i16", "int16_t"),
        ("i32", "int32_t"),
        ("i64", "int64_t"),
        ("isize", "intptr_t"),
        //
        ("bool", "bool"),
    ] {
        header += &vec_of_primitive_headers(rust_ty, c_ty);
    }

    header
}

/// Headers for Vec<T> where T is a primitive such as u8, i32, bool
fn vec_of_primitive_headers(rust_ty: &str, c_ty: &str) -> String {
    let mut chars = rust_ty.chars();

    // u8 -> U8, bool -> Bool, etc...
    let capatilized_first_letter =
        chars.next().unwrap().to_string().to_uppercase() + chars.as_str();

    // __private__OptionU8 ... etc
    let option_ty = format!("{}{}", "__private__Option", capatilized_first_letter);

    format!(
        r#"
void* __swift_bridge__$Vec_{rust_ty}$new();
void __swift_bridge__$Vec_{rust_ty}$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_{rust_ty}$len(void* const vec);
void __swift_bridge__$Vec_{rust_ty}$push(void* const vec, {c_ty} val);
{option_ty} __swift_bridge__$Vec_{rust_ty}$pop(void* const vec);
{option_ty} __swift_bridge__$Vec_{rust_ty}$get(void* const vec, uintptr_t index);
{option_ty} __swift_bridge__$Vec_{rust_ty}$get_mut(void* const vec, uintptr_t index);
{c_ty} const * __swift_bridge__$Vec_{rust_ty}$as_ptr(void* const vec);
"#,
        rust_ty = rust_ty,
        c_ty = c_ty,
        option_ty = option_ty
    )
}

fn conform_to_vectorizable(swift_ty: &str, rust_ty: &str) -> String {
    format!(
        r#"
extension {swift_ty}: Vectorizable {{
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {{
        __swift_bridge__$Vec_{rust_ty}$new()
    }}

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {{
        __swift_bridge__$Vec_{rust_ty}$_free(vecPtr)
    }}

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {{
        __swift_bridge__$Vec_{rust_ty}$push(vecPtr, value)
    }}

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {{
        let val = __swift_bridge__$Vec_{rust_ty}$pop(vecPtr)
        if val.is_some {{
            return val.val
        }} else {{
            return nil
        }}
    }}

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        let val = __swift_bridge__$Vec_{rust_ty}$get(vecPtr, index)
        if val.is_some {{
            return val.val
        }} else {{
            return nil
        }}
    }}

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        let val = __swift_bridge__$Vec_{rust_ty}$get_mut(vecPtr, index)
        if val.is_some {{
            return val.val
        }} else {{
            return nil
        }}
    }}

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {{
        __swift_bridge__$Vec_{rust_ty}$len(vecPtr)
    }}
}}
    "#,
        rust_ty = rust_ty,
        swift_ty = swift_ty
    )
}

/// Used to free memory for generic Opaque Rust types such as `type SomeType<u32>`
fn generic_freer() -> &'static str {
    r#"
protocol SwiftBridgeGenericFreer {
    func rust_free();
}
    "#
}

/// A Swift protocol that is implemented for the FFI representation of all generic Copy types
/// such as `#[swift_bride(Copy(4))] type SomeType<u32>`
fn generic_copy_type_ffi_repr() -> &'static str {
    r#"
protocol SwiftBridgeGenericCopyTypeFfiRepr {}
"#
}
