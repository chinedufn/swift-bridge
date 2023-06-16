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
    /// Verify that we change the Swift name of a transparent enum.
    func testSharedEnumSwiftName() throws {
        XCTAssertEqual(
            extern_rust_enum_rename(
                EnumRename.Variant1
            ),
            EnumRename.Variant1
        )
    }
    
    
    /// Verify that we can call a rust function from swift that uses a type that was already declared in a different bridge module.
    func testSharedEnumAlreadyDeclared() throws {
        XCTAssertEqual(
            rust_reflect_already_declared_enum(
                AlreadyDeclaredEnumTest.Variant
            ),
            AlreadyDeclaredEnumTest.Variant
        )
    }
    
    /// Verify that we can call a swift function from rust that uses a type that was already declared in a different bridge module.
    func testRustCallsSwiftFunctionSharedEnumAlreadyDeclared() throws {
        test_rust_calls_swift_already_declared_enum()
    }
    
    /// Verify that we can use the generated Debug impl.
    func testSharedEnumDeriveDebug() throws {
        let debugString = String(reflecting: DeriveDebugEnum.Variant)
        XCTAssertEqual(debugString, "Variant")
    }

}

