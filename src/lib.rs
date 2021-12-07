//! Generate FFI glue between Swift and Rust code.

#![deny(missing_docs)]

pub use swift_bridge_macro::bridge;

mod std_bridge;

pub use self::std_bridge::{option, string};

#[doc(hidden)]
#[repr(C)]
pub struct FfiSlice<T> {
    pub start: *const T,
    pub len: usize,
}

// Unlike the Swift pointer wrapper types that we generate, this type does not implement drop.
// So we can freely construct it and pass it over the FFI boundary without worrying about drop
//
// It has the same layout as the __private__PointerToSwift C struct, so when we pass this to
// Swift it can receive it as a __private__PointerToSwift.
#[doc(hidden)]
#[repr(C)]
pub struct PointerToSwiftType(pub *mut std::ffi::c_void);

impl<T> FfiSlice<T> {
    /// Create an FfiSlice from a slice.
    pub fn from_slice(slice: &[T]) -> Self {
        FfiSlice {
            start: slice.as_ptr(),
            len: slice.len(),
        }
    }

    /// Get a reference to the slice that this FfiSlice points to.
    pub fn as_slice(&self) -> &'static [T] {
        unsafe { std::slice::from_raw_parts(self.start, self.len) }
    }
}
