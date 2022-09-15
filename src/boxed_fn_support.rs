#![allow(non_snake_case)]

#[export_name = "__swift_bridge__$call_boxed_fn_once_no_args_no_return"]
pub extern "C" fn __swift_bridge__call_boxed_fn_once_no_args_no_return(
    boxed_fn: *mut Box<dyn FnOnce() -> ()>,
) {
    unsafe { Box::from_raw(boxed_fn)() };
}

#[export_name = "__swift_bridge__$free_boxed_fn_once_no_args_no_return"]
pub extern "C" fn __swift_bridge__free_boxed_fn_once_no_args_no_return(
    boxed_fn: *mut Box<dyn FnOnce() -> ()>,
) {
    unsafe {
        let _ = Box::from_raw(boxed_fn);
    }
}
