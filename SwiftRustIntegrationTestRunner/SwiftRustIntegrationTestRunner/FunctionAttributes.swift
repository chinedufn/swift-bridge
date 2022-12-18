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
