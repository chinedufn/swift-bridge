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

    /// Verify that extern "Swift" functions with unlabeled params (`label = "_"`) work.
    func testSwiftFuncWithUnlabeledParams() throws {
        // The generated wrapper should call: swift_func_with_unlabeled_params(a, b)
        XCTAssertEqual(swift_func_with_unlabeled_params(10, 20), 30)
    }

    /// Verify that extern "Swift" functions with custom labels work.
    func testSwiftFuncWithCustomLabels() throws {
        // The generated wrapper should call: swift_func_with_custom_labels(first: a, second: b)
        XCTAssertEqual(swift_func_with_custom_labels(first: 5, second: 3), 8)
    }

}
