//! The corresponding C and Swift code can be found in
//! crates/swift-bridge-build/src/generate_core/rust_string.{c.h,swift}
pub use self::ffi::*;

#[swift_bridge_macro::bridge(swift_bridge_path = crate)]
mod ffi {
    extern "Rust" {
        type RustString;

        #[swift_bridge(init)]
        fn new() -> RustString;

        #[swift_bridge(init)]
        fn new_with_str(str: &str) -> RustString;

        fn len(&self) -> usize;

        fn as_str(&self) -> &str;

        fn trim(&self) -> &str;
    }
}

#[doc(hidden)]
pub struct RustString(pub String);

#[doc(hidden)]
#[repr(C)]
pub struct RustStr {
    pub start: *const u8,
    pub len: usize,
}

impl RustString {
    fn new() -> Self {
        RustString("".to_string())
    }

    fn new_with_str(str: &str) -> Self {
        RustString(str.to_string())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn trim(&self) -> &str {
        self.0.trim()
    }
}

impl RustString {
    /// Box::into_raw(Box::new(self))
    pub fn box_into_raw(self) -> *mut RustString {
        Box::into_raw(Box::new(self))
    }
}

impl RustStr {
    pub fn len(&self) -> usize {
        self.len
    }

    // TODO: Think through these lifetimes and the implications of them...
    pub fn to_str<'a>(self) -> &'a str {
        let bytes = unsafe { std::slice::from_raw_parts(self.start, self.len) };
        std::str::from_utf8(bytes).expect("Failed to convert RustStr to &str")
    }

    pub fn to_string(self) -> String {
        self.to_str().to_string()
    }

    pub fn from_str(str: &str) -> Self {
        RustStr {
            start: str.as_ptr(),
            len: str.len(),
        }
    }

    pub fn partial_eq(
        lhs: Self,
        rhs: Self,
    ) -> bool {
    
        unsafe { 
            std::slice::from_raw_parts(lhs.start, lhs.len) ==
            std::slice::from_raw_parts(rhs.start, rhs.len)
        }
    }
}
