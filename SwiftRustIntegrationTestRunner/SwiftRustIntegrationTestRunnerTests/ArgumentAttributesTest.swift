//
//  ArgumentAttributes.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Niwaka on 2023/02/04.
//

import Foundation

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests argument attributes such as `#[swift_bridge(label = "someArg")]`.
final class ArgumentAttributesTest: XCTestCase {

    /// Verify that the `swift_bridge(label = "someArg")` attribute works.
    func testArgumentLabel() throws {
        XCTAssertEqual(test_argument_label(someArg: 10, 100), 110)
    }

    /// Verify that Rust can call extern "Swift" functions with unlabeled params (`label = "_"`).
    func testRustCallsSwiftWithUnlabeledParams() throws {
        // Rust function calls Swift: swift_func_with_unlabeled_params(10, 20)
        XCTAssertEqual(rust_calls_swift_with_unlabeled_params(10, 20), 30)
    }

    /// Verify that Rust can call extern "Swift" functions with custom labels.
    func testRustCallsSwiftWithCustomLabels() throws {
        // Rust function calls Swift: swift_func_with_custom_labels(first: 5, second: 3)
        XCTAssertEqual(rust_calls_swift_with_custom_labels(5, 3), 8)
    }

}
