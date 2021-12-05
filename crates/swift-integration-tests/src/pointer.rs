use std::ffi::c_void;

#[swift_bridge::bridge]
mod ffi {
    // Opaque pointers.
    extern "Rust" {
        fn rust_echo_const_c_void(ptr: *const c_void) -> *const c_void;
        fn rust_echo_mut_c_void(ptr: *mut c_void) -> *mut c_void;

        fn rust_echo_const_u8(ptr: *const u8) -> *const u8;
        fn rust_echo_mut_u8(ptr: *mut u8) -> *mut u8;

        fn rust_run_opaque_pointer_tests();
        fn rust_run_u8_pointer_tests();
    }

    // Opaque pointers.
    extern "Swift" {
        fn swift_echo_const_c_void(ptr: *const c_void) -> *const c_void;
        fn swift_echo_mut_c_void(ptr: *mut c_void) -> *mut c_void;

        fn swift_echo_const_u8(ptr: *const u8) -> *const u8;
        fn swift_echo_mut_u8(ptr: *mut u8) -> *mut u8;
    }
}

/// Verify that we can pass and return opaque pointers across the boundary.
fn rust_run_opaque_pointer_tests() {
    let num = &123;
    let num_mut = &mut 555;

    let ptr = num as *const i32 as *const c_void;
    let ptr_mut = num_mut as *mut i32 as *mut c_void;

    let ptr_copy = ffi::swift_echo_const_c_void(ptr);
    let ptr_mut_copy = ffi::swift_echo_mut_c_void(ptr_mut);

    assert_eq!(unsafe { *(ptr_copy as *const i32) }, 123);

    assert_eq!(unsafe { *(ptr_mut_copy as *mut i32) }, 555);
}

/// Verify that we can pass and return u8 pointers across the boundary.
fn rust_run_u8_pointer_tests() {
    let num = &5u8;
    let num_mut = &mut 10u8;

    let ptr = num as *const u8;
    let ptr_mut = num_mut as *mut u8;

    let ptr_copy = ffi::swift_echo_const_u8(ptr);
    let ptr_mut_copy = ffi::swift_echo_mut_u8(ptr_mut);

    assert_eq!(unsafe { *ptr_copy }, 5);

    assert_eq!(unsafe { *ptr_mut_copy }, 10);
}

fn rust_echo_const_c_void(ptr: *const c_void) -> *const c_void {
    ptr
}

fn rust_echo_mut_c_void(ptr: *mut c_void) -> *mut c_void {
    ptr
}

fn rust_echo_const_u8(ptr: *const u8) -> *const u8 {
    ptr
}

fn rust_echo_mut_u8(ptr: *mut u8) -> *mut u8 {
    ptr
}
