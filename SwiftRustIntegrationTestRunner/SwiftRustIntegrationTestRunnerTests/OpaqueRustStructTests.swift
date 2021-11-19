//
//  OpaqueRustStructTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/14/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class OpaqueRustStructTests: XCTestCase {
    
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    /// Verify that we can create and work with an opaque Rust struct via a generated class
    func testOpaqueRustStruct() throws {
        let stack = ARustStack()
        
        XCTAssertEqual(stack.len(), 0)
        stack.push(5)
        stack.push(10)
        XCTAssertEqual(stack.len(), 2)
        
        XCTAssertEqual(stack.as_slice()[0], 5)
        XCTAssertEqual(stack.as_slice()[1], 10)
        
        stack.pop()
        XCTAssertEqual(stack.len(), 1)
    }
}
