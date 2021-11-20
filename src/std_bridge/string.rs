pub use self::ffi::*;

// TODO: Also look for `#[swift_bridge_macro]` in -build crate.

#[swift_bridge_macro::bridge(swift_bridge_path = crate)]
mod ffi {
    extern "Rust" {
        type RustString;

        #[swift_bridge(init)]
        fn new() -> RustString;

        #[swift_bridge(init)]
        fn new_with_str(str: &str) -> RustString;

        fn len(&self) -> usize;

        fn trim(&self) -> &str;
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

    pub fn to_str(&self) -> &str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn as_bytes(&self) -> &'static [u8] {
        unsafe { std::slice::from_raw_parts(self.start, self.len) }
    }

    pub fn from_str(str: &str) -> Self {
        RustStr {
            start: str.as_ptr(),
            len: str.len(),
        }
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
