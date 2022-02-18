//
//  SwiftRustIntegrationTestRunnerTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/14/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class StringTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testRunRustCallsSwiftTests() throws {
        run_string_tests()
    }
    
    /// Verify that we can get a RustString's length
    func testRustStringLen() throws {
        let string = " hello "
        let rustString: RustString = create_string(string)
        
        XCTAssertEqual(rustString.len(), 7)
    }
    
    /// Verify that we can trim a RustString
    func testTrimRustString() throws {
        let string = " hello "
        let rustString: RustString = create_string(string)
        
        let trimmed: RustStr = rustString.trim()
        XCTAssertEqual(trimmed.len, 5)
    }
    
    func testRustStringToString() throws {
        let string = "hi"
        
        XCTAssertEqual(
            create_string(string).toString(),
            "hi"
        )
    }
}


