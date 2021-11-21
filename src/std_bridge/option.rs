// Ok.. time to plan Option<T>
//
// How would we return an `Option<Vec<u32>>`?
//
// We can return `*mut Vec<u32>` if Some, or a `null` pointer if none.
// Similarly.. we can receive the arg as a pointer and check if it's null.
//
// So.. that's easy. What about for primitives?
// We could box them into pointers and then do the same null pointer trick.. But ideally we'd
// avoid allocations.
//
// Say you have MyStruct { foo: u8 }
// Ideally `Option<MyStruct>` would not allocate.
// MyStruct would have a corresponding C struct. We'd want that to be treated the same as any other
// Copy type.
// Which is fine.. because during codegen we would know the generated the same stuff the our
// primitives use.
//
// Ok so.. for every type we know the type's size. Even if that isn't true.. we can just limit
// options to types with known size.
// So.. if we have some sort of global flag "The last value returned is `None`" ... then we'd
// just check that flag.
// This would need to be thread local.
// So... say Swift calls RustFuncA .. The caller and callee live on the same thread.. So if we had
// a thread local variable for options we'd be able to look it up.
// So `static mut OPTION_ARGS: [bool; 256] = [false; 256];`
// And `static mut OPTION_RETURN: bool = false;`
// Later OPTION_ARGS could use a bitflag instead of an array.

use std::cell::Cell;
const CELL: Cell<bool> = Cell::new(false);

thread_local! {
    pub static OPTION_ARGS: [Cell<bool>; 256] = [CELL; 256];
    pub static OPTION_RETURN: Cell<bool> = CELL;
}

#[doc(hidden)]
pub trait FfiOptional {
    /// Used to create a value of this type that won't actually be used by the other side of the
    /// FFI boundary since we've set a flag to instruct Swift to ignore what we return and use
    /// `None` (Swift calls it `nil`) instead.
    fn unused_value() -> Self;
}

impl FfiOptional for u8 {
    fn unused_value() -> Self {
        123
    }
}

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
