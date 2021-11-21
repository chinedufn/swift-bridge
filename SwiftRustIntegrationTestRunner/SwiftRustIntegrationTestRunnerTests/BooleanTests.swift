//
//  BooleanTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/20/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class BooleanTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testRustTests() throws {
        runBoolTest()
    }
    
    func testSwiftTests() throws {
        XCTAssertEqual(rustNegateBool(true), false)
        XCTAssertEqual(rustNegateBool(false), true)
    }
}
