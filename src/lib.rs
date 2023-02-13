//! Generate FFI glue between Swift and Rust code.

#![deny(missing_docs)]

pub use swift_bridge_macro::bridge;

mod std_bridge;

pub use self::std_bridge::{option, result, string};

#[doc(hidden)]
#[cfg(feature = "async")]
pub mod async_support;

#[doc(hidden)]
pub mod boxed_fn_support;

#[doc(hidden)]
pub mod copy_support;

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

// The code generation automatically implements this for all shared structs.
// This trait is private and should not be used outside of swift-bridge.
//
// The main use case is for structs that use the `#[swift_bridge(already_declared)]`
// attribute, where we use `<SomeStruct as SharedStruct::FfiRepr>` to get the
// struct's FFI representation.
#[doc(hidden)]
pub trait SharedStruct {
    /// The FFI friendly representation of this struct.
    ///
    /// ```
    /// struct MyStruct {
    ///     field: &'static str
    /// }
    /// // This is the auto generated ffi representation.
    /// #[repr(C)]
    /// struct __swift_bridge__MyStruct {
    ///     field: swift_bridge::string::RustStr
    /// }
    /// ```
    type FfiRepr;
}

// The code generation automatically implements this for all shared enum.
// This trait is private and should not be used outside of swift-bridge.
//
// The main use case is for enums that use the `#[swift_bridge(already_declared)]`
// attribute, where we use `<SomeEnum as SharedEnum::FfiRepr>` to get the
// enum's FFI representation.
#[doc(hidden)]
pub trait SharedEnum {
    /// The FFI friendly representation of this enum.
    ///
    /// ```
    /// enum MyEnum {
    ///     Variant1,
    ///     Variant2,
    /// }
    /// // This is the auto generated ffi representation.
    /// #[repr(C)]
    /// enum __swift_bridge__MyEnum {
    ///     Variant1,
    ///     Variant2,
    /// }
    /// ```
    type FfiRepr;
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn __swift_bridge__null_pointer() -> *const std::ffi::c_void {
    std::ptr::null()
}
