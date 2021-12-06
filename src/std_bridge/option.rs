use std::cell::Cell;

const CELL: Cell<bool> = Cell::new(false);

thread_local! {
    pub static OPTION_ARGS: [Cell<bool>; 256] = [CELL; 256];
    pub static OPTION_RETURN: Cell<bool> = CELL;
}

use self::macro_::impl_ffi_optional;

// TODO: Can probably ditch this trait and instead just add a method in `BuiltInType` that generates
//  a placeholder value.
#[doc(hidden)]
pub trait FfiOptional {
    /// Used to create a value of this type that won't actually be used by the other side of the
    /// FFI boundary since we've set a flag to instruct Swift to ignore what we return and use
    /// `None` (Swift calls it `nil`) instead.
    fn unused_value() -> Self;
}

impl_ffi_optional!(u8, 123);
impl_ffi_optional!(u16, 123);
impl_ffi_optional!(u32, 123);
impl_ffi_optional!(u64, 123);
impl_ffi_optional!(usize, 123);

impl_ffi_optional!(i8, 123);
impl_ffi_optional!(i16, 123);
impl_ffi_optional!(i32, 123);
impl_ffi_optional!(i64, 123);
impl_ffi_optional!(isize, 123);

impl_ffi_optional!(bool, false);

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn _get_option_arg(idx: u8) -> bool {
    OPTION_ARGS.with(|o| o[idx as usize].get())
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn _set_option_arg(idx: u8, bool: bool) {
    OPTION_ARGS.with(|o| o[idx as usize].set(bool));
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn _get_option_return() -> bool {
    OPTION_RETURN.with(|o| o.get())
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn _set_option_return(bool: bool) {
    OPTION_RETURN.with(|o| o.set(bool));
}

mod macro_ {
    macro_rules! impl_ffi_optional {
        ($ty:ty, $val:expr) => {
            impl FfiOptional for $ty {
                fn unused_value() -> Self {
                    $val
                }
            }
        };
    }

    pub(super) use impl_ffi_optional;
}
