#[swift_bridge::bridge]
mod ffi {
    extern "Swift" {
        fn swift_takes_fnonce_callback_no_args_no_return(arg: Box<dyn FnOnce() -> ()>);
        fn swift_takes_fnonce_callback_primitive(arg: Box<dyn FnOnce(u8) -> u8>) -> u8;
        fn swift_takes_fnonce_callback_opaque_rust(
            arg: Box<dyn FnOnce(CallbackTestOpaqueRustType) -> CallbackTestOpaqueRustType>,
        );

        fn swift_takes_two_fnonce_callbacks(
            arg1: Box<dyn FnOnce()>,
            arg2: Box<dyn FnOnce(u8) -> u16>,
        ) -> u16;
        fn swift_takes_fnonce_callback_with_two_params(arg: Box<dyn FnOnce(u8, u16) -> u16>)
            -> u16;

        fn swift_calls_rust_fnonce_callback_twice(arg: Box<dyn FnOnce() -> ()>);

        fn swift_func_takes_callback_with_result_arg(
            arg: Box<dyn FnOnce(Result<CallbackTestOpaqueRustType, String>)>,
        );
    }

    extern "Swift" {
        type SwiftMethodCallbackTester;

        #[swift_bridge(init)]
        fn new() -> SwiftMethodCallbackTester;

        fn method_with_fnonce_callback(&self, callback: Box<dyn FnOnce() -> ()>);
        fn method_with_fnonce_callback_primitive(
            &self,
            callback: Box<dyn FnOnce(u16) -> u16>,
        ) -> u16;
    }

    // TODO
    // extern "Rust" {
    //     fn rust_takes_callback_fnonce_no_args_no_return(arg: Box<dyn FnOnce() -> ()>);
    //     fn rust_takes_callback_fnonce_primitive(doubling_fn: Box<dyn FnOnce(u8) -> u8>);
    //     fn rust_takes_callback_fnonce_opaque_rust(
    //         doubling_fn: Box<dyn FnOnce(CallbackTestOpaqueRustType) -> CallbackTestOpaqueRustType>,
    //     );
    //
    //     fn rust_takes_callback_fnonce_two_params(
    //         arg: Box<dyn FnOnce(i16, CallbackTestOpaqueRustType)>,
    //     );
    //
    //     fn rust_takes_two_callbacks_fnonce_noop(
    //         arg1: Box<dyn FnOnce()>,
    //         arg2: Box<dyn FnOnce() -> ()>,
    //     );
    // }

    extern "Rust" {
        type CallbackTestOpaqueRustType;

        #[swift_bridge(init)]
        fn new(val: u32) -> CallbackTestOpaqueRustType;
        fn val(&self) -> u32;
        fn double(&mut self);
    }

    extern "Rust" {
        fn test_callbacks_rust_calls_swift();
    }
}

// TODO
// fn rust_takes_callback_fnonce_no_args_no_return(arg: Box<dyn FnOnce() -> ()>) {
//     (arg)()
// }
// fn rust_takes_callback_fnonce_primitive(doubling_fn: Box<dyn FnOnce(u8) -> u8>) {
//     let doubling_fn = (doubling_fn)(2);
//     assert_eq!(doubled, 4)
// }
//
// fn rust_takes_callback_fnonce_opaque_rust(
//     doubling_fn: Box<dyn FnOnce(CallbackTestOpaqueRustType) -> CallbackTestOpaqueRustType>,
// ) {
//     let start = CallbackTestOpaqueRustType { val: 100 };
//
//     let doubled = (doubling_fn)(start);
//     assert_eq!(doubled.val(), 200);
// }
//
// fn rust_takes_callback_fnonce_two_params(arg: Box<dyn FnOnce(i16, CallbackTestOpaqueRustType)>) {
//     (arg)(123, CallbackTestOpaqueRustType { val: 222 })
// }
// fn rust_takes_two_callbacks_fnonce_noop(arg: Box<dyn FnOnce(i16, CallbackTestOpaqueRustType)>) {
//     (arg)(123, CallbackTestOpaqueRustType { val: 222 })
// }

pub struct CallbackTestOpaqueRustType {
    val: u32,
}
impl CallbackTestOpaqueRustType {
    pub fn new(val: u32) -> Self {
        Self { val }
    }

    pub fn val(&self) -> u32 {
        self.val
    }

    pub fn double(&mut self) {
        self.val *= 2
    }
}

fn test_callbacks_rust_calls_swift() {
    let swift_callback_tester = ffi::SwiftMethodCallbackTester::new();

    ffi::swift_takes_fnonce_callback_no_args_no_return(Box::new(|| {}));

    let four_times_two = ffi::swift_takes_fnonce_callback_primitive(Box::new(|num| num * 2));
    assert_eq!(four_times_two, 8);

    ffi::swift_takes_fnonce_callback_opaque_rust(Box::new(|mut rust_ty| {
        rust_ty.double();
        rust_ty
    }));

    ffi::swift_takes_fnonce_callback_with_two_params(Box::new(|_num1, num2| num2 * 2));

    let three_times_two =
        ffi::swift_takes_two_fnonce_callbacks(Box::new(|| {}), Box::new(|num| (num * 2) as u16));
    assert_eq!(three_times_two, 6);

    swift_callback_tester.method_with_fnonce_callback(Box::new(|| {}));

    let five_times_two =
        swift_callback_tester.method_with_fnonce_callback_primitive(Box::new(|num| num * 2));
    assert_eq!(five_times_two, 10);

    ffi::swift_func_takes_callback_with_result_arg(Box::new(|result| {
        assert_eq!(result.unwrap().val(), 555)
    }));
}
