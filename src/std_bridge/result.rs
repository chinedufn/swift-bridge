#[repr(C)]
#[doc(hidden)]
// Bridges `Result<T, E>` where `T` and `E` are non primitive types.
pub struct ResultPtrAndPtr {
    pub is_ok: bool,
    pub ok_or_err: *mut std::ffi::c_void,
}

// TODO: We need to define every combination of primitive and pointer.
//  Probably low priority since most users are probably using non-primitive types for
//  `Result<T, E>`.
//
// #[repr(C)]
// #[doc(hidden)]
// pub struct ResultU8AndU8 {
//     pub is_ok: bool,
//     pub ok_or_err: u8,
// }
//
// #[repr(C)]
// #[doc(hidden)]
// pub struct ResultU8AndU16 {
//     pub is_ok: bool,
//     pub ok: u8,
//     pub err: u16
// }
//
// #[repr(C)]
// #[doc(hidden)]
// pub struct ResultU8AndPtr {
//     pub is_ok: bool,
//     pub ok: u8,
//     pub err: *mut std::ffi::c_void,
// }
