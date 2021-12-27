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

    func testSwiftString() throws {
        run_string_tests()
    }
    
    func testRustString() throws {
        let string = " hello "
        
        let rustString: RustString = create_string(string.toRustStr())
        XCTAssertEqual(rustString.len(), 7)
        
        let trimmed: RustStr = rustString.trim()
        XCTAssertEqual(trimmed.len, 5)
        
        XCTAssertEqual(rustString.toString(), string)
    }
}
