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

    /// Verify that we can pass and return a Tuple.
    func testSwiftCallsRustTuples() throws {
        XCTContext.runActivity(named: "primitive types") {
            _ in
            let tuple = reflect_tuple_primitive_types((-1, 10))
            XCTAssertEqual(tuple.0, -1)
            XCTAssertEqual(tuple.1, 10)
        }
        XCTContext.runActivity(named: "String and primitive type") {
            _ in
            //let tuple = reflect_tuple_string_and_primitive_type(("hello", 10))
            //XCTAssertEqual(tuple.0.toString(), "hello")
            //XCTAssertEqual(tuple.1, 10)
        }
    }
    
}
