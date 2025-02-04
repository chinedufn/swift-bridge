//
//  SendableAttributeTests.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 2/4/25.
//

import XCTest
@testable import SwiftRustIntegrationTestRunner

/// Tests for the `#[swift_bridge(Sendable)]` attribute.
///
/// For the corresponding Rust code, see `crates/swift-integration-tests/src/sendable_attribute.rs`.
/// For the corresponding codegen tests, see `crates/swift-bridge-ir/src/codegen/codegen_tests/sendable_attribute.rs`.
class SendableAttributeTests: XCTestCase {
    /// Verify that we can a Rust type that has the `#[swift_bridge(Sendable)]` attribute gets a `Sendable` protocol implementation.
    func testSendableExternRustType() throws {
        let sendableRustType = SendableRustType()
        
        // Move the type to another thread.
        Thread.detachNewThread {
            let _ = sendableRustType;
        }
    }
}


protocol AssertIsSendable: Sendable {}
extension SendableRustType: AssertIsSendable{}

