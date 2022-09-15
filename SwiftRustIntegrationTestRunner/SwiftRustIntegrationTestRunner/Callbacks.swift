//
//  Callbacks.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 9/11/22.
//

import Foundation

func swift_takes_fnonce_callback_no_args_no_return(arg: () -> ()) {
    arg()
}

func swift_takes_fnonce_callback_primitive(
    arg: (UInt8) -> UInt8
) -> UInt8 {
    arg(4)
}

func swift_takes_fnonce_callback_opaque_rust(
    arg: (CallbackTestOpaqueRustType) -> CallbackTestOpaqueRustType
) {
    let doubled = arg(CallbackTestOpaqueRustType(10))
    if doubled.val() != 20 {
        fatalError("Callback not called")
    }
}

func swift_takes_two_fnonce_callbacks(
    arg1: () -> (),
    arg2: (UInt8) -> UInt16
) -> UInt16 {
    arg1()
    return arg2(3)
}

func swift_takes_fnonce_callback_with_two_params(
    arg: (UInt8, UInt16) -> UInt16
) -> UInt16 {
    arg(1, 2)
}

/// When given an FnOnce callback this should panic.
func swift_calls_rust_fnonce_callback_twice(arg: () -> ()) {
    arg()
    arg()
}

class SwiftMethodCallbackTester {
    func method_with_fnonce_callback(callback: () -> ()) {
        callback()
    }
    
    func method_with_fnonce_callback_primitive(callback: (UInt16) -> UInt16) -> UInt16 {
        callback(5)
    }
}
