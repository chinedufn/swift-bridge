//
//  TupleTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Niwaka on 2023/03/12.
//

import Foundation

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests tuples
final class TupleTest: XCTestCase {
    /// Verify that we can pass and return Rust tuples.
    func testSwiftCallsRustTuples() throws {
        XCTContext.runActivity(named: "Verify that we can pass and return a (primitive type, primitive type).") {
            _ in
            let tuple = rust_reflect_tuple_primitives((-1, 10))
            XCTAssertEqual(tuple.0, -1)
            XCTAssertEqual(tuple.1, 10)
        }
        XCTContext.runActivity(named: "Verify that we can pass and return a (OpaqueRustType, String, primitive type).") {
            _ in
            let tuple = rust_reflect_tuple_opaque_rust_and_string_and_primitive((TupleTestOpaqueRustType(123), "foo", 128))
            XCTAssertEqual(tuple.0.val(), 123)
            XCTAssertEqual(tuple.1.toString(), "foo")
            XCTAssertEqual(tuple.2, 128)
        }
        XCTContext.runActivity(named: "Verify that we can pass and return a (F64, UInt, Bool).") {
            _ in
            let tuple = rust_reflect_tuple_f64_and_usize_and_bool((0.1, 123, true))
            XCTAssertEqual(tuple.0, 0.1)
            XCTAssertEqual(tuple.1, 123)
            XCTAssertEqual(tuple.2, true)
        }
    }
    
    /// Verify that Rust can call Swift functions that accept and return Tuples.
    func testRustCallsSwiftTuple() throws {
        test_rust_calls_swift_tuples()
    }
}
