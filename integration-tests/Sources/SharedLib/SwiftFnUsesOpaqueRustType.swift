//
//  SwiftFnUsesOpaqueRustType.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/28/21.
//

import Foundation

func increment_some_owned_opaque_rust_type(arg: SomeRustType, amount: UInt32) {
    arg.increment_counter(amount)
}
