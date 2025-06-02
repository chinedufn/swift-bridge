pub(super) fn swift_option_primitive_support() -> String {
    let types = [
        ("U8", "UInt8", "123"),
        ("I8", "Int8", "123"),
        ("U16", "UInt16", "123"),
        ("I16", "Int16", "123"),
        ("U32", "UInt32", "123"),
        ("I32", "Int32", "123"),
        ("U64", "UInt64", "123"),
        ("I64", "Int64", "123"),
        ("Usize", "UInt", "123"),
        ("Isize", "Int", "123"),
        ("F32", "Float", "123.4"),
        ("F64", "Double", "123.4"),
        ("Bool", "Bool", "false"),
    ];
    let mut all = "".to_string();

    for (suffix, inner_ty, unused_none) in types {
        let option_ffi_ty = format!("__private__Option{suffix}");

        all += &format!(
            r#"
extension {option_ffi_ty} {{
    func intoSwiftRepr() -> Optional<{inner_ty}> {{
        if self.is_some {{
            return self.val 
        }} else {{
            return nil
        }}
    }}

    init(_ val: Optional<{inner_ty}>) {{
        if let val = val {{
            self = Self(val: val, is_some: true) 
        }} else {{
            self = Self(val: {unused_none}, is_some: false) 
        }}
    }}
}}
extension Optional where Wrapped == {inner_ty} {{
    func intoFfiRepr() -> {option_ffi_ty} {{
        {option_ffi_ty}(self) 
    }}
}}
"#
        );
    }

    all
}

pub(super) const C_OPTION_PRIMITIVE_SUPPORT: &str = r#"
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
typedef struct __private__OptionF64 { double val; bool is_some; } __private__OptionF64;
typedef struct __private__OptionBool { bool val; bool is_some; } __private__OptionBool;
"#;
