//
//  SwiftFnUsesOpaqueSwiftTypeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 8/23/22.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class SwiftFnReturnOpaqueSwiftTypeTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testSwiftFnReturnOpaqueSwiftType() throws {
        test_rust_calls_swift_fn_reflects_owned_opaque_swift_type()
    }
    
    func testSwiftMethodReturnOpaqueSwiftType() throws {
        test_rust_calls_swift_method_reflects_owned_opaque_swift_type()
    }
}
