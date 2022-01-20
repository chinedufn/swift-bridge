//
//  SharedStructAttributeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 1/6/22.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests for attributes on shared structs types.
class SharedStructAttributeTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    /// Verify that we can call a function that uses a type that was already declared in a different bridge module.
    /// See crates/swift-integration-tests/src/struct_attributes/already_declared.rs
    func testSharedStructAlreadyDeclaredCallInitializer() throws {
        let val = AlreadyDeclaredStructTest(field: 123)
        
        XCTAssertEqual(
            reflect_already_declared_struct(val).field,
            123
        )
    }
}

