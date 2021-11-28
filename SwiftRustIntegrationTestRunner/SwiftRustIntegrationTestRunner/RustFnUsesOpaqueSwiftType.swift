//
//  RustFnReturnOpaqueSwiftType.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/27/21.
//

import Foundation

/// We expose this to the `rust_function_return_swift_type.rs` test.
class SomeSwiftType {
    var text: String
    
    init() {
        text = "initial text"
    }
    
    func setText(text: RustStr) {
        self.text = text.toString()
    }
}
