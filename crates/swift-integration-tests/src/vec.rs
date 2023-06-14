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

    extern "Rust" {
        fn run_vec_tests();
    }

    extern "Swift" {
        fn receive_bytes() -> Vec<u8>;
        fn send_bytes(vec: Vec<u8>);
    }
}

fn run_vec_tests() {
    let vec = ffi::receive_bytes();
    assert_eq!(vec.len(), 5);
    assert_eq!(vec[0], 0);
    assert_eq!(vec[1], 1);
    assert_eq!(vec[2], 2);
    assert_eq!(vec[3], 3);
    assert_eq!(vec[4], 4);

    let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
    ffi::send_bytes(vec);
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
