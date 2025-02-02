//
//  Primitive.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 10/3/22.
//

import Foundation

func swift_double_u8(arg: UInt8) -> UInt8 {
    arg * 2
}

func swift_double_i8(arg: Int8) -> Int8 {
    arg * 2
}

func swift_double_u16(arg: UInt16) -> UInt16 {
    arg * 2
}

func swift_double_i16(arg: Int16) -> Int16 {
    arg * 2
}

func swift_double_u32(arg: UInt32) -> UInt32 {
    arg * 2
}

func swift_double_i32(arg: Int32) -> Int32 {
    arg * 2
    
}

func swift_double_u64(arg: UInt64) -> UInt64 {
    arg * 2
}

func swift_double_i64(arg: Int64) -> Int64 {
    arg * 2
}

func swift_double_f32(arg: Float) -> Float {
    arg * 2.0
}

func swift_double_f64(arg: Double) -> Double {
    arg * 2.0
}

func swift_negate_bool(arg: Bool) -> Bool {
    !arg
}

func swift_reflect_null(arg: ()) -> () {
    arg
}
