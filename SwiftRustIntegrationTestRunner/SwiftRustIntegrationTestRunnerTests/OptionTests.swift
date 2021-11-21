//
//  OptionTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/21/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class OptionTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testSwiftCallRustReturnOption() throws {
        XCTAssertEqual(create_rust_option_u8_some(), 70)
        XCTAssertEqual(create_rust_option_u8_none(), nil)
    }
    
    func testRustCallSwiftReturnOption() {
        run_option_tests()
    }
}



