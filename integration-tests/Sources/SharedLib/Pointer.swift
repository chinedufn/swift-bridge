//
//  Pointer.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/23/21.
//

import Foundation


func swift_echo_const_c_void(ptr: UnsafeRawPointer) -> UnsafeRawPointer {
    ptr
}
func swift_echo_mut_c_void(ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    ptr
}

func swift_echo_const_u8(ptr: UnsafePointer<UInt8>) -> UnsafePointer<UInt8> {
    ptr
}
func swift_echo_mut_u8(ptr: UnsafeMutablePointer<UInt8>) -> UnsafeMutablePointer<UInt8> {
    ptr
}
