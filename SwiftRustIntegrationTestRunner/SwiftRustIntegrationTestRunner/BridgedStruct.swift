//
//  BridgedStruct.swift
//  SwiftRustIntegrationTestRunner
//
//  Tests for #[swift_bridge::bridged] attribute macro
//

import Foundation

// Error conformance required for throwing functions
extension BridgedError: @unchecked Sendable {}
extension BridgedError: Error {}

// Swift functions called by Rust

func swift_create_bridged_response(success: Bool, count: UInt32) -> BridgedResponse {
    return BridgedResponse(success: success, count: count)
}

func swift_fallible_bridged_response(succeed: Bool) throws(BridgedError) -> BridgedResponse {
    if succeed {
        return BridgedResponse(success: true, count: 99)
    } else {
        throw BridgedError.InvalidInput("Swift error".intoRustString())
    }
}
