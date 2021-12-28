use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=SWIFT_BRIDGE_OUT_DIR");

    if let Ok(out_dir) = std::env::var("SWIFT_BRIDGE_OUT_DIR") {
        let out_dir = PathBuf::from(out_dir);

        let generated = swift_bridge_build::parse_bridges(vec![
            //
            manifest_dir().join("src/std_bridge/string.rs"),
        ]);
        let generated_swift = generated.concat_swift();
        let generated_c = generated.concat_c();

        let core_swift_out = out_dir.join("SwiftBridgeCore.swift");
        let mut swift = core_swift();
        swift += "\n";
        swift += &generated_swift;

        std::fs::write(core_swift_out, swift).unwrap();

        let core_c_header_out = out_dir.join("SwiftBridgeCore.h");
        let mut c_header = core_c_header().to_string();
        c_header += "\n";
        c_header += &generated_c;

        std::fs::write(core_c_header_out, c_header).unwrap();
    }
}

fn core_swift() -> String {
    let mut core_swift = "".to_string();

    core_swift += include_str!("src/std_bridge/string.swift");
    core_swift += include_str!("src/std_bridge/rust_vec.swift");

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

    core_swift
}

fn core_c_header() -> String {
    let mut header = r#"#include <stdint.h>
#include <stdbool.h> 
typedef struct RustStr { uint8_t* const start; uintptr_t len; } RustStr;
typedef struct __private__FfiSlice { void* const start; uintptr_t len; } __private__FfiSlice;
typedef struct __private__PointerToSwiftType { void* ptr; } __private__RustHandleToSwiftType;
bool _get_option_arg(uint8_t is_some);
void _set_option_arg(uint8_t idx, bool is_some);
bool _get_option_return();
void _set_option_return(bool is_some);
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
    format!(
        r#"
void* __swift_bridge__$Vec_{rust_ty}$new();
void __swift_bridge__$Vec_{rust_ty}$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_{rust_ty}$len(void* const vec);
void __swift_bridge__$Vec_{rust_ty}$push(void* const vec, {c_ty} val);
{c_ty} __swift_bridge__$Vec_{rust_ty}$pop(void* const vec);
{c_ty} __swift_bridge__$Vec_{rust_ty}$get(void* const vec, uintptr_t index);
{c_ty} __swift_bridge__$Vec_{rust_ty}$get_mut(void* const vec, uintptr_t index);
{c_ty} const * __swift_bridge__$Vec_{rust_ty}$as_ptr(void* const vec);
"#,
        rust_ty = rust_ty,
        c_ty = c_ty
    )
}

fn conform_to_vectorizable(swift_ty: &str, rust_ty: &str) -> String {
    format!(
        r#"
extension {swift_ty}: Vectorizable {{
    static func vecOfSelfNew() -> UnsafeMutableRawPointer {{
        __swift_bridge__$Vec_{rust_ty}$new()
    }}

    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {{
        __swift_bridge__$Vec_{rust_ty}$_free(vecPtr)
    }}

    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self) {{
        __swift_bridge__$Vec_{rust_ty}$push(vecPtr, value)
    }}

    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {{
        __swift_bridge__$Vec_{rust_ty}$pop(vecPtr)
    }}

    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        __swift_bridge__$Vec_{rust_ty}$get(vecPtr, index)
    }}

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self> {{
        __swift_bridge__$Vec_{rust_ty}$get_mut(vecPtr, index)
    }}

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {{
        __swift_bridge__$Vec_{rust_ty}$len(vecPtr)
    }}
}}
    "#,
        rust_ty = rust_ty,
        swift_ty = swift_ty
    )
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
