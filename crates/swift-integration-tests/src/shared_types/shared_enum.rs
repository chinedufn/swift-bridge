#[swift_bridge::bridge]
mod ffi {
    enum EnumWithNoData {
        Variant1,
        Variant2,
    }

    extern "Rust" {
        fn reflect_enum_with_no_data(arg: EnumWithNoData) -> EnumWithNoData;
    }
}

fn reflect_enum_with_no_data(arg: ffi::EnumWithNoData) -> ffi::EnumWithNoData {
    arg
}
