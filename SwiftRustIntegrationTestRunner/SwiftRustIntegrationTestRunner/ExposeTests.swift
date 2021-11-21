//
//  ExposeTests.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/20/21.
//

import Foundation

// Not sure how to let the SwiftRustIntegrationTestsRunnerTests target see our
// linked external functions that come from Rust.. So exposing them here.
// TODO: Look into this.

public func runStringTests () {
    run_string_tests()
}

public func runOpaqueSwiftClassTests () {
    run_opaque_swift_class_tests()
}

public func createRustString (str: RustStr) -> RustString {
    create_string(str)
}





