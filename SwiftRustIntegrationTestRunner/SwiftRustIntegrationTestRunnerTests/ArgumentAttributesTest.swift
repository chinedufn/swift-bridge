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

}
