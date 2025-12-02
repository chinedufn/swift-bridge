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

extension SwiftAsyncVoidError: @unchecked Sendable {}
extension SwiftAsyncVoidError: Error {}

extension SwiftAsyncMethodError: @unchecked Sendable {}
extension SwiftAsyncMethodError: Error {}

// ============================================================================
// Sync Swift throwing functions (called from Rust)
// ============================================================================

extension SwiftSyncError: @unchecked Sendable {}
extension SwiftSyncError: Error {}

/// Sync Swift function that throws, returning u32 on success
/// Uses typed throws (Swift 5.9+) to ensure compile-time verification of error type
func swift_sync_throws_u32(succeed: Bool) throws(SwiftSyncError) -> UInt32 {
    if succeed {
        return 42
    } else {
        throw SwiftSyncError.ErrorWithValue(123)
    }
}

/// Sync Swift function that throws, returning String on success
/// Uses typed throws (Swift 5.9+) to ensure compile-time verification of error type
func swift_sync_throws_string(succeed: Bool) throws(SwiftSyncError) -> RustString {
    if succeed {
        return "Success from Swift".intoRustString()
    } else {
        throw SwiftSyncError.ErrorWithMessage("Error message from Swift".intoRustString())
    }
}
