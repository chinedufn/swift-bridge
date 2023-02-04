//
//  ArgumentAttributes.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Niwaka on 2023/02/04.
//

import Foundation

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests the #[swift_bridge(label = "someArg")] attribute.
final class ArgumentAttributesTest: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    /// Verify that the `swift_bridge(label = "someArg")` attribute works.
    func testArgumentLabel() throws {
        XCTAssertEqual(test_argument_label(someArg: 10, 100), 110)
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }

}
