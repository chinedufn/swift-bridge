//
//  Result.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 9/20/22.
//

func swift_func_takes_callback_with_result_arg(
    arg: (RustResult<CallbackTestOpaqueRustType, String>) -> ()
) {
    arg(.Ok(CallbackTestOpaqueRustType(555)))
}
