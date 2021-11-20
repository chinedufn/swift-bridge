//
//  SwiftRustIntegrationTestRunnerTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/14/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class SwiftRustIntegrationTestRunnerTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testSwiftStrings() throws {
        runStringTests()
    }
    
    func testRustStrings() throws {
        let string = " hello "
        
        let rustString: RustString = createRustString(str: string.toRustStr())
        XCTAssertEqual(rustString.len(), 7)
        
        let trimmed: RustStr = rustString.trim()
        XCTAssertEqual(trimmed.len, 5)
    }
}
