#[repr(C)]
#[doc(hidden)]
pub struct OptionU8 {
    pub val: u8,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionI8 {
    pub val: i8,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionU16 {
    pub val: u16,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionI16 {
    pub val: i16,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionU32 {
    pub val: u32,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionI32 {
    pub val: i32,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionU64 {
    pub val: u64,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionI64 {
    pub val: i64,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionUsize {
    pub val: usize,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionIsize {
    pub val: isize,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionF32 {
    pub val: f32,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionF64 {
    pub val: f64,
    pub is_some: bool,
}

#[repr(C)]
#[doc(hidden)]
pub struct OptionBool {
    pub val: bool,
    pub is_some: bool,
}
