use std::cell::Cell;

const CELL: Cell<bool> = Cell::new(false);

thread_local! {
    pub static OPTION_ARGS: [Cell<bool>; 256] = [CELL; 256];
    pub static OPTION_RETURN: Cell<bool> = CELL;
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
