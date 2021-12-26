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

    func testSwiftCallRustReturnOptionU8() throws {
        XCTAssertEqual(create_rust_option_u8_some(), 70)
        XCTAssertEqual(create_rust_option_u8_none(), nil)
    }
    
    func testSwiftCallRustReturnOptionString() throws {
        let string = create_rust_option_string_some()!
        XCTAssertEqual(string.trim().toString(), "hello world")
        XCTAssertNil(create_rust_option_string_none())
    }
    
    func testRustCallSwiftReturnOption() {
        run_option_tests()
    }
}



