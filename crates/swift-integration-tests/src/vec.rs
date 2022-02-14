#[swift_bridge::bridge]
mod ffi {
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
