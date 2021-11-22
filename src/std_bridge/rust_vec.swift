// TODO:
//  Implement iterator https://developer.apple.com/documentation/swift/iteratorprotocol

class RustVec<T: Vectorizable & FfiOption> {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool

    init(ptr: UnsafeMutableRawPointer, isOwned: Bool) {
        self.ptr = ptr
        self.isOwned = isOwned
    }

    init() {
        ptr = T.vecOfSelfNew()
        isOwned = true
    }

    func push (value: T) {
        T.vecOfSelfPush(vecPtr: ptr, value: value)
    }

    func pop () -> Optional<T> {
        let val = T.vecOfSelfPop(vecPtr: ptr)
        if _get_option_return() {
            return val;
        } else {
            return nil
        }
    }

    func get(index: UInt) -> Optional<T> {
        let val = T.vecOfSelfGet(vecPtr: ptr, index: index)
        if _get_option_return() {
            return val;
        } else {
            return nil
        }
    }

    func len() -> UInt {
        T.vecOfSelfLen(vecPtr: ptr)
    }

    func asPtr() -> UnsafePointer<T> {
        T.vecOfSelfAsPtr(vecPtr: ptr)
    }

    func toUnsafeBufferPointer() -> UnsafeBufferPointer<T> {
        UnsafeBufferPointer(start: asPtr(), count: Int(len()))
    }

    deinit {
         T.vecOfSelfFree(vecPtr: ptr)
    }
}

extension RustVec: Sequence {
    func makeIterator() -> RustVecIterator<T> {
        return RustVecIterator(self)
    }
}

struct RustVecIterator<T: Vectorizable & FfiOption>: IteratorProtocol {
    var rustVec: RustVec<T>
    var index: UInt = 0

    init (_ rustVec: RustVec<T>) {
        self.rustVec = rustVec
    }

    mutating func next() -> T? {
        let val = rustVec.get(index: index)
        index += 1
        return val
    }
}


extension UnsafeBufferPointer {
    func toFfiSlice () -> __private__FfiSlice {
        __private__FfiSlice(start: UnsafeMutablePointer(mutating: self.baseAddress), len: UInt(self.count))
    }
}

extension Array {
    func toUnsafeBufferPointer() -> UnsafeBufferPointer<Element> {
        UnsafeBufferPointer(start: UnsafePointer(self), count: self.count)
    }
}

protocol Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer;

    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer)

    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self)

    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self>

    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Self>

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt

    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self>
}
