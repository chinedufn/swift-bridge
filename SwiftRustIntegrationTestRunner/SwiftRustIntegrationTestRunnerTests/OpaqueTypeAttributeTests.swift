//
//  OpaqueTypeAttributeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 1/6/22.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests for attributes on opaque types.
class OpaqueTypeAttributeTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    /// Verify that we can call an initializers, methods and associated functions that were declared in a different module from
    /// where the opaque Rust type was defined.
    /// This ensures that our code generation properly generates Swift convenience initializers inside of class extensions.
    /// See crates/swift-integration-tests/src/type_attributes/already_declared.rs
    func testExternRustAlreadyDeclaredOpaqueRustType() throws {
        let val = AlreadyDeclaredTypeTest()
        
        XCTAssert(val.a_ref_method())
        XCTAssert(val.a_ref_mut_method())
        XCTAssert(val.an_owned_method())
        
        XCTAssert(AlreadyDeclaredTypeTest.an_associated_function())
    }
    
    func testExternRustAlreadyDeclaredCopyOpaqueRustTypeType() throws {
        let val = AlreadyDeclaredCopyTypeTest()
        
        XCTAssert(val.a_ref_method())
        XCTAssert(val.an_owned_method())
        
        XCTAssert(AlreadyDeclaredCopyTypeTest.an_associated_function())
    }

    
    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }

}
