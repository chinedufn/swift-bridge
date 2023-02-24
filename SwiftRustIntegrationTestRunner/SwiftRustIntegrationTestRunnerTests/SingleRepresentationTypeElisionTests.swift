//
//  SingleRepresentationTypeElisionTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 2/23/23.
//

import Foundation

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Test that we can call functions that have elided single representation types.
/// See:
///   - crates/swift-bridge-ir/src/codegen/codegen_tests/single_representation_type_elision_codegen_tests.rs
///   - crates/swift-integration-tests/src/single_representation_type_elision.rs
final class SingleRepresentationTypeElisionTest: XCTestCase {
    /// Verify that we can call functions that take the null type.
    func testSwiftCallsRustNullType() throws {
        let _: () = rust_one_null_arg(())
        let _: () = rust_two_null_args((), ())
    }
    
    /// Verify that we can call functions that take a unit struct.
    func testSwiftCallsRustUnitStruct() throws {
        let _: SingleReprTestUnitStruct = rust_one_unit_struct(SingleReprTestUnitStruct())
        let _: SingleReprTestUnitStruct = rust_two_unit_structs(SingleReprTestUnitStruct(), SingleReprTestUnitStruct())
    }
}
