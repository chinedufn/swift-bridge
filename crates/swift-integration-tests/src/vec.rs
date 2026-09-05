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
        #[swift_bridge(Copy(4))]
        type ARustCopyTypeInsideVecT;

        #[swift_bridge(init)]
        fn new(value: i32) -> ARustCopyTypeInsideVecT;

        fn value(&self) -> i32;
    }

    extern "Rust" {
        fn rust_reflect_vec_opaque_rust_copy_type(
            arg: Vec<ARustCopyTypeInsideVecT>,
        ) -> Vec<ARustCopyTypeInsideVecT>;
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
        fn swift_return_vec_u8() -> Vec<u8>;
        fn swift_arg_vec_u8(vec: Vec<u8>);
    }
}

fn run_vec_tests() {
    let vec = ffi::swift_return_vec_u8();
    assert_eq!(vec.len(), 5);
    assert_eq!(vec[0], 0);
    assert_eq!(vec[1], 1);
    assert_eq!(vec[2], 2);
    assert_eq!(vec[3], 3);
    assert_eq!(vec[4], 4);

    let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
    ffi::swift_arg_vec_u8(vec);
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

#[derive(Clone, Copy)]
pub struct ARustCopyTypeInsideVecT {
    x: i32,
}

impl ARustCopyTypeInsideVecT {
    fn new(x: i32) -> Self {
        Self { x }
    }

    fn value(&self) -> i32 {
        self.x
    }
}

fn rust_reflect_vec_opaque_rust_copy_type(arg: Vec<ARustCopyTypeInsideVecT>) -> Vec<ARustCopyTypeInsideVecT> {
    arg
}

fn rust_reflect_vec_transparent_enum(
    arg: Vec<ffi::TransparentEnumInsideVecT>,
) -> Vec<ffi::TransparentEnumInsideVecT> {
    arg
}
