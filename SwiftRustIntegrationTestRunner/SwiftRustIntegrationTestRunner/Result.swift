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

public class ResultTestOpaqueSwiftType {
    var num: UInt32
    
    init(val: UInt32) {
        self.num = val
    }
    
    func val() -> UInt32 {
        self.num
    }
}

extension ResultTestOpaqueRustType: @unchecked Sendable {}
extension ResultTestOpaqueRustType: Error {}

extension AsyncResultOpaqueRustType1: @unchecked Sendable {}
extension AsyncResultOpaqueRustType1: Error {}

extension AsyncResultOpaqueRustType2: @unchecked Sendable {}
extension AsyncResultOpaqueRustType2: Error {}

extension ResultTransparentEnum: @unchecked Sendable {}
extension ResultTransparentEnum: Error {}

extension SameEnum: @unchecked Sendable {}
extension SameEnum: Error {}

extension AsyncResultErrEnum: @unchecked Sendable {}
extension AsyncResultErrEnum: Error {}
