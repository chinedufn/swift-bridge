//
//  String.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 2/18/22.
//

import Foundation

func create_swift_string() -> String {
    "hello"
}

func reflect_rust_string(arg: RustString) -> RustString {
    arg
}
