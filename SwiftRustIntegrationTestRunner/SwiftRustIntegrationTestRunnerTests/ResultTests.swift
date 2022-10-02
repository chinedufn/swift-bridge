//
//  ResultTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 9/20/22.

import XCTest
@testable import SwiftRustIntegrationTestRunner

class ResultTests: XCTestCase {
    /// Verify that we can pass a Result<String, String> from Swift -> Rust
    func testSwiftCallRustResultString() throws {
        rust_func_takes_result_string(.Ok("Success Message"))
        rust_func_takes_result_string(.Err("Error Message"))
    }
    
    /// Verify that we can pass a Result<OpaqueRust, OpaqueRust> from Swift -> Rust
    func testSwiftCallRustResultOpaqueRust() throws {
        rust_func_takes_result_opaque_rust(
            .Ok(ResultTestOpaqueRustType(111))
        )
        rust_func_takes_result_opaque_rust(
            .Err(ResultTestOpaqueRustType(222))
        )
    }
    
    /// Verify that we can pass a Result<OpaqueSwift, OpaqueSwift> from Swift -> Rust
    func testSwiftCallRustResultOpaqueSwift() throws {
        rust_func_takes_result_opaque_swift(
            .Ok(ResultTestOpaqueSwiftType(val: 555))
        )
        rust_func_takes_result_opaque_swift(
            .Err(ResultTestOpaqueSwiftType(val: 666))
        )
    }
}
