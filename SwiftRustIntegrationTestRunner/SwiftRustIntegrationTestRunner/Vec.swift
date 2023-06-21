import Foundation

func swift_arg_vec_u8(vec: RustVec<UInt8>) {
    assert(vec[0] == 1)
    assert(vec[1] == 2)
    assert(vec[2] == 3)
    assert(vec[3] == 4)
    assert(vec[4] == 5)
}

func swift_return_vec_u8() -> RustVec<UInt8> {
    let vec = RustVec<UInt8>()
    for i in 0 ... 4 {
        vec.push(value: UInt8(i))
    }
    return vec
}
