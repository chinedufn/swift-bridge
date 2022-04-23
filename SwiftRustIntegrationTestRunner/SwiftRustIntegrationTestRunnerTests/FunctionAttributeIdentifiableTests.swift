//
//  FunctionAttributeIdentifiableTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 1/27/22.
//

import Foundation

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests the #[swift_bridge(Identifiable)] attribute.
class FunctionAttributeIdentifiableTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    /// Verify that the `swift_bridge(Identifiable)` attribute works.
    func testIdentifiable() throws {
        XCTAssertEqual(verifyIsIdentifiable(IdentifiableFnNamedId()).id(), 123)
        XCTAssertEqual(IdentifiableFnNotNamedId().id, 123)
        XCTAssertEqual(OpaqueCopyTypeIdentifiable().id(), 123)
        
        XCTAssertEqual(verifyIsIdentifiable(IdentifiableU8()).id(), 123)
        XCTAssertEqual(verifyIsIdentifiable(IdentifiableI8()).id(), 123)
        
        "hello world".toRustStr({rustStr in
            XCTAssertEqual(verifyIsIdentifiable(IdentifiableStr()).id(), rustStr)
        })
    }
}

func verifyIsIdentifiable<T: Identifiable>(_ arg: T) -> T {
    arg
}
