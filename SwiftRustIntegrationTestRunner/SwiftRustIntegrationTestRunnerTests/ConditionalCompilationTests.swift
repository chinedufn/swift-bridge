//
//  SwiftFnUsesOpaqueRustTypeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/28/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner


class ConditionalCompilationTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    /// Call a function from a bridge module that is only exposed when the
    /// "this_is_enabled" feature is enabled for the Rust crate.
    /// We have that feature enabled by default, so we should be able to call the function.
    func testConditionalCompilation() throws {
        XCTAssertEqual(conditionally_exposed_fn(), 123)
    }
}
