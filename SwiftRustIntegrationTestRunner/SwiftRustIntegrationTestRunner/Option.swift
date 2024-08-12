//
//  Option.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/21/21.
//

import Foundation

func swift_reflect_option_u8(arg: Optional<UInt8>) -> Optional<UInt8> {
    arg
}
func swift_reflect_option_i8(arg: Optional<Int8>) -> Optional<Int8> {
    arg
}
func swift_reflect_option_u16(arg: Optional<UInt16>) -> Optional<UInt16> {
    arg
}
func swift_reflect_option_i16(arg: Optional<Int16>) -> Optional<Int16> {
    arg
}
func swift_reflect_option_u32(arg: Optional<UInt32>) -> Optional<UInt32> {
    arg
}
func swift_reflect_option_i32(arg: Optional<Int32>) -> Optional<Int32> {
    arg
}
func swift_reflect_option_u64(arg: Optional<UInt64>) -> Optional<UInt64> {
    arg
}
func swift_reflect_option_i64(arg: Optional<Int64>) -> Optional<Int64> {
    arg
}
func swift_reflect_option_usize(arg: Optional<UInt>) -> Optional<UInt> {
    arg
}
func swift_reflect_option_isize(arg: Optional<Int>) -> Optional<Int> {
    arg
}
func swift_reflect_option_f32(arg: Optional<Float>) -> Optional<Float> {
    arg
}
func swift_reflect_option_f64(arg: Optional<Double>) -> Optional<Double> {
    arg
}
func swift_reflect_option_bool(arg: Optional<Bool>) -> Optional<Bool> {
    arg
}


func swift_reflect_option_string(arg: Optional<RustString>) -> Optional<RustString> {
    arg
}
// TODO: Change `swift_arg_option_str` `swift_reflect_option_str` once we support Swift returning `-> &str` via RustStr
// For now we return true if the arg is Some and false if arg is None
//func swift_reflect_option_str(arg: Optional<RustStr>) -> Optional<RustStr> {
//    arg
//}
func swift_arg_option_str(arg: Optional<RustStr>) -> Bool {
    if let val = arg {
        assert(val.toString() == "this is an option str")
        return true
    } else {
        return false
    }
}

public class OptTestOpaqueSwiftType {
    let val: Int

    init(val: Int) {
        self.val = val
    }
}
