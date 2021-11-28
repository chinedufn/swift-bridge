//
//  SwiftFnUsesOpaqueRustTypeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/28/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner


class SwiftFnUsesOpaqueRustTypeTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testRustFnCallsWithFnWithOwnedOpaqueArg() throws {
        test_call_swift_fn_with_owned_opaque_rust_arg()
    }
}
