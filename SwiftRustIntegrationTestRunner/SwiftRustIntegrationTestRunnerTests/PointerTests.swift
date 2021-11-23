//
//  PointerTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/22/21.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

class PointerTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testSwiftCallRustCvoid() throws {
        let value = [1, 2, 3]
        
        let pointer = UnsafeRawPointer(value)
        let pointer_mut = UnsafeMutableRawPointer(mutating: value)
        
        let pointer_copy = rust_echo_const_c_void(pointer)
        let pointer_mut_copy = rust_echo_mut_c_void(pointer_mut)
        
        XCTAssertEqual(pointer, pointer_copy)
        XCTAssertEqual(pointer_mut, pointer_mut_copy)
    }
    
    func testRustCallSwiftCvoid() throws {
        rust_run_opaque_pointer_tests()
    }
    
    func testRustCallSwiftUInt8() throws {
        rust_run_u8_pointer_tests()
    }
}
