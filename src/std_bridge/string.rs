//! The corresponding C and Swift code can be found in
//! crates/swift-bridge-build/src/generate_core/rust_string.{c.h,swift}
pub use self::ffi::*;

/// Ideally, we would bridge Rust's [`String`] to Swift's `String` type directly.
/// We do not do this because there is no zero-copy way to create a Swift `String`, and, since
/// `swift-bridge` aims to be useful in performance sensitive applications, we avoid unnecessary
/// allocations.
///
/// Instead, users that wish to go from a `Rust std::string::String` to a Swift String must call
/// `RustString.toString()` on the Swift side.
/// We can consider introducing annotations that allow a user to opt in to an automatic conversion.
/// For instance, something along the lines of:
/// ```rust,no_run
/// #[swift_bridge::bridge]
/// mod ffi {
///     extern "Rust" {
///         #[swift_bridge(return_clone)]
///         fn return_a_string() -> String;
///
///         #[swift_bridge(return_map_ok_clone)]
///         fn return_a_string_ok() -> Result<String, ()>;
///
///         #[swift_bridge(return_map_err_clone)]
///         fn return_a_string_err() -> Result<(), String>;
///     }
/// }
/// ```
/// When such an attribute was present `swift-bridge` would allocate a Swift String on the Swift
/// side, instead of initializing an instance of the `RustString` class.
///
/// Such an automatic conversion could be made more efficient than using the `RustString.toString()`
/// method to create a Swift String.
/// For instance, to go from `Rust std::string::String -> Swift String` via a `RustString` we:
/// - allocate a `class RustString` instance
/// - call `RustString.toString()`, which constructs a Swift String using the `RustString`'s
///    underlying buffer
///
/// An automatic conversion would look like:
/// - construct a Swift String using the Rust `std::string::String`'s underlying buffer
///
/// Regardless of whether one is using `swift-bridge`, creating instances of Swift reference types
/// requires a small heap allocation.
/// By not creating an instance of the `RustString` class we would be eliminating one small
/// allocation.
///
/// ## References
/// - claim: Impossible to create a Swift `String` without copying:
///   - `init(bytesNoCopy was deprecated in macOS 13` - https://forums.swift.org/t/init-bytesnocopy-was-deprecated-in-macos-13/61231
///   - "String does not support no-copy initialization" - https://developer.apple.com/documentation/swift/string/init(bytesnocopy:length:encoding:freewhendone:)
///   - `Does String(bytesNoCopy:) copy bytes?` - https://forums.swift.org/t/does-string-bytesnocopy-copy-bytes/51643
/// - claim: Class instances allocate
///   - "For example, a class instance (which allocates)" https://www.swift.org/documentation/server/guides/allocations.html#other-perf-tricks
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
}

impl PartialEq for RustStr {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            std::slice::from_raw_parts(self.start, self.len)
                == std::slice::from_raw_parts(other.start, other.len)
        }
    }
}

#[export_name = "__swift_bridge__$RustStr$partial_eq"]
#[allow(non_snake_case)]
pub extern "C" fn __swift_bridge__RustStr_partial_eq(lhs: RustStr, rhs: RustStr) -> bool {
    lhs == rhs
}
