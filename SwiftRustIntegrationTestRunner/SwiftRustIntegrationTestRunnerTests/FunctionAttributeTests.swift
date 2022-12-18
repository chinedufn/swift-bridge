//
// FunctionAttributeIdentifiableTests.swift
// SwiftRustIntegrationTestRunnerTests
//
// Created by Erik Živković on 2022-12-17.
//

import Foundation

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests the #[swift_bridge(swift_name = "x")] attribute.
class FunctionAttributeTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    /// Verify that the `swift_bridge(swift_name = "x")` attribute works.
    func testSwiftNameAttribute() throws {
        XCTAssertEqual(testCallRustFromSwiftByNameAttribute().toString(), "StringFromRust")
    }
}
