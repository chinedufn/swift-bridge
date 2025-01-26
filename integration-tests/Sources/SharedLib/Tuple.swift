//
//  Tuple.swift
//  SwiftRustIntegrationTestRunner
//
//  Created by Niwaka on 2023/03/25.
//

import Foundation

func swift_reflect_tuple_primitives(arg: (Int32, UInt32)) -> (Int32, UInt32) {
    arg
}

func swift_reflect_opaque_and_primitive_tuple(arg: (TupleTestOpaqueRustType, Int32)) -> (TupleTestOpaqueRustType, Int32) {
    arg
}

func swift_reflect_struct_and_enum_and_string(arg: (TupleTestStruct, TupleTestEnum, RustString)) -> (TupleTestStruct, TupleTestEnum, RustString) {
    arg
}
