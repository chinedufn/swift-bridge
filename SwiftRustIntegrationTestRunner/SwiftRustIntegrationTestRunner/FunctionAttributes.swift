//
//  FunctionAttributes.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Erik Živković on 2022-12-17.
//

import Foundation

func testCallSwiftFromRustByNameAttribute() -> RustString {
    return "StringFromSwift".intoRustString()
}

// MARK: - Argument Label Tests

/// Swift function with unlabeled parameters (using `_`)
/// Called from Rust as: swift_func_with_unlabeled_params(10, 20)
func swift_func_with_unlabeled_params(_ a: Int32, _ b: Int32) -> Int32 {
    return a + b
}

/// Swift function with custom parameter labels
/// Called from Rust as: swift_func_with_custom_labels(first: 5, second: 3)
func swift_func_with_custom_labels(first a: Int32, second b: Int32) -> Int32 {
    return a + b
}
