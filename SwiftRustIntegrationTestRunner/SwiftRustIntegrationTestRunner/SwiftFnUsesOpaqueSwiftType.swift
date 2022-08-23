//
//  SwiftFnUsesOpaqueSwiftType.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 8/23/22.
//

import Foundation

class ASwiftType {
    private var _amount: UInt32
    
    init(amount: UInt32) {
        self._amount = amount
    }
    
    func amount() -> UInt32 {
        self._amount
    }
    
    func call_swift_method_reflects_owned_opaque_swift_type(arg: ASwiftType) -> ASwiftType {
        arg
    }
}

func call_swift_fn_reflects_owned_opaque_swift_type(arg: ASwiftType) -> ASwiftType {
    arg
}

