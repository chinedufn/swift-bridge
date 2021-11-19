use swift_bridge_macro as swift_bridge;

pub use self::ffi::*;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type RustString;

        #[swift_bridge(init)]
        fn new() -> RustString;

        #[swift_bridge(init)]
        fn new_with_string() -> RustString;

        fn len(&self) -> usize;
    }

    extern "Swift" {
        type SwiftString;

        #[swift_bridge(init)]
        fn new() -> SwiftString;

        #[swift_bridge(init)]
        fn new_with_str(str: &str) -> SwiftString;

        fn as_ptr(&self) -> *const u8;

        fn len(&self) -> usize;
    }
}

struct RustString(String);

impl RustString {
    fn new() -> Self {
        RustString("".to_string())
    }

    fn new_with_str(&self, str: &str) -> Self {
        RustString(str.to_string())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl SwiftString {
    pub fn as_bytes(&self) -> &[u8] {
        let ptr = self.as_ptr();
        let len = self.len();

        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    pub fn to_str(&self) -> &str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }
}
