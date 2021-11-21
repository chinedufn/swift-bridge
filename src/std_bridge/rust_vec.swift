class RustVec<T: Vectorizable> {
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

    func pop () {
        T.vecOfSelfPop(vecPtr: ptr)
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

extension UnsafeBufferPointer {
    func toFfiSlice () -> __private__FfiSlice {
        __private__FfiSlice(start: UnsafeMutablePointer(mutating: self.baseAddress), len: UInt(self.count))
    }
}

protocol Vectorizable {
    static func vecOfSelfNew() -> UnsafeMutableRawPointer;

    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer)

    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self)

    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer)

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt

    static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<Self>
}
