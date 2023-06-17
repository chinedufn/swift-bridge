import Foundation

func send_bytes(vec _: RustVec<UInt8>) {}

func receive_bytes() -> RustVec<UInt8> {
    let vec = RustVec<UInt8>()
    for i in 0 ... 4 {
        vec.push(value: UInt8(i))
    }
    return vec
}
