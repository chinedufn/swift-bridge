//
//  OpaqueRustStructTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/14/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class OpaqueSwiftStructTests: XCTestCase {
    
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    /// Run the Rust tests that create and use an opaque Swift struct.
    func testOpaqueSwiftStruct() throws {
        run_opaque_swift_class_tests()
    }
}
