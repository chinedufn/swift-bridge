//
//  SharedEnumAttributeTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 12/15/22.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests for attributes on transparent enum types.
class SharedEnumAttributeTests: XCTestCase {
    /// Verify that we can call a function that uses a type that was already declared in a different bridge module.
    func testSharedEnumAlreadyDeclared() throws {
        XCTAssertEqual(
            reflect_already_declared_enum(
                AlreadyDeclaredEnumTest.Variant
            ),
            AlreadyDeclaredEnumTest.Variant
        )
    }
}
