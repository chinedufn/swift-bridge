#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type ARustTypeInsideVecT;
        fn text(&self) -> &str;
    }

    extern "Rust" {
        fn create_vec_u8(contents: &[u8]) -> Vec<u8>;
        fn create_vec_i32(contents: &[i32]) -> Vec<i32>;

        fn create_vec_opaque_rust_type() -> Vec<ARustTypeInsideVecT>;
    }
}

struct ARustTypeInsideVecT {
    text: String,
}

impl ARustTypeInsideVecT {
    fn text(&self) -> &str {
        &self.text
    }
}

fn create_vec_u8(contents: &[u8]) -> Vec<u8> {
    contents.to_vec()
}

fn create_vec_i32(contents: &[i32]) -> Vec<i32> {
    contents.to_vec()
}

fn create_vec_opaque_rust_type() -> Vec<ARustTypeInsideVecT> {
    vec![ARustTypeInsideVecT {
        text: "hello there, friend".to_string(),
    }]
}
