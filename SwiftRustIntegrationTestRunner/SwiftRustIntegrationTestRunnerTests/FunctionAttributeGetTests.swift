//
//  FunctionAttributeGetTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 5/10/22.
//

import Foundation

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests the #[swift_bridge(get(...))] attribute.
class FunctionAttributeGetTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    /// Verify that the `swift_bridge(get(...))` attribute works for a Option<&'static str>.
    func testGetOptStaticStr() throws {
        let val = SomeTypeGet()
        XCTAssertEqual(val.my_opt_static_str()!.toString(), "world")
    }
}

