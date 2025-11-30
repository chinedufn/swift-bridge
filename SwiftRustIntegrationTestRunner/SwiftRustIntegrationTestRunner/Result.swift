//
//  Result.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 9/20/22.
//

func swift_func_takes_callback_with_result_arg(
        arg: (RustResult<CallbackTestOpaqueRustType, String>) -> Void
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

// TODO: we can delete these type assertions once we correctly generate Sendable
// types. See the following issue:
// https://github.com/chinedufn/swift-bridge/issues/150

extension AsyncRustFnReturnStruct: @unchecked Sendable {}

extension ResultTestOpaqueRustType: @unchecked Sendable {}
extension ResultTestOpaqueRustType: Error {}

extension AsyncResultOpaqueRustType1: @unchecked Sendable {}
extension AsyncResultOpaqueRustType1: Error {}

extension AsyncResultOpaqueRustType2: @unchecked Sendable {}
extension AsyncResultOpaqueRustType2: Error {}

extension ResultTransparentEnum: @unchecked Sendable {}
extension ResultTransparentEnum: Error {}

extension ResultTransparentStruct: @unchecked Sendable {}
extension ResultTransparentStruct: Error {}

extension SameEnum: @unchecked Sendable {}
extension SameEnum: Error {}

extension AsyncResultOkEnum: @unchecked Sendable {}

extension AsyncResultErrEnum: @unchecked Sendable {}
extension AsyncResultErrEnum: Error {}

extension SwiftAsyncError: @unchecked Sendable {}
extension SwiftAsyncError: Error {}
