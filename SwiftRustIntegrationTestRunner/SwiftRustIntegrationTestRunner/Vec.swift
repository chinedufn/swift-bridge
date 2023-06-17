import Foundation

func swift_arg_vec_u8(vec _: RustVec<UInt8>) {}

func swift_return_vec_u8() -> RustVec<UInt8> {
    let vec = RustVec<UInt8>()
    for i in 0 ... 4 {
        vec.push(value: UInt8(i))
    }
    return vec
}
