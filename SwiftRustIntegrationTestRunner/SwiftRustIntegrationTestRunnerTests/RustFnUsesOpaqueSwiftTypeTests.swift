//
//  RustFnReturnOpaqueSwiftTypeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/27/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class RustFnReturnOpaqueSwiftTypeTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testRustFnReturnOpaqueSwiftType() throws {
        let someSwiftType = rust_fn_return_opaque_swift_type()
        XCTAssertEqual(someSwiftType.text, "I was initialized from Rust")
    }
}
