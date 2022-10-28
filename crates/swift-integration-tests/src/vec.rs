#[swift_bridge::bridge]
mod ffi {
    enum TransparentEnumInsideVecT {
        VariantA,
        VariantB,
    }

    extern "Rust" {
        type ARustTypeInsideVecT;

        #[swift_bridge(init)]
        fn new(text: &str) -> ARustTypeInsideVecT;

        fn text(&self) -> &str;
    }

    extern "Rust" {
        fn rust_reflect_vec_opaque_rust_type(
            arg: Vec<ARustTypeInsideVecT>,
        ) -> Vec<ARustTypeInsideVecT>;
    }

    extern "Rust" {
        fn rust_reflect_vec_transparent_enum(
            arg: Vec<TransparentEnumInsideVecT>,
        ) -> Vec<TransparentEnumInsideVecT>;
    }
}

pub struct ARustTypeInsideVecT {
    text: String,
}

impl ARustTypeInsideVecT {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    fn text(&self) -> &str {
        &self.text
    }
}

fn rust_reflect_vec_opaque_rust_type(arg: Vec<ARustTypeInsideVecT>) -> Vec<ARustTypeInsideVecT> {
    arg
}

fn rust_reflect_vec_transparent_enum(
    arg: Vec<ffi::TransparentEnumInsideVecT>,
) -> Vec<ffi::TransparentEnumInsideVecT> {
    arg
}
