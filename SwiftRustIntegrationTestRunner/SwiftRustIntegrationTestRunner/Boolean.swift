//
//  Boolean.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/20/21.
//

import Foundation

public func runBoolTest() {
    run_bool_test()
}

public func rustNegateBool(_ bool: Bool) -> Bool {
    rust_negate_bool(bool)
}

public func swiftNegateBool(start: Bool) -> Bool {
    !start
}
