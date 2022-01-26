// TODO:
//  Implement iterator https://developer.apple.com/documentation/swift/iteratorprotocol

class RustVec<T: Vectorizable> {
    var ptr: UnsafeMutableRawPointer
    var isOwned: Bool = true

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    init() {
        ptr = T.vecOfSelfNew()
        isOwned = true
    }

    func push (value: T) {
        T.vecOfSelfPush(vecPtr: ptr, value: value)
    }

    func pop () -> Optional<T> {
        T.vecOfSelfPop(vecPtr: ptr)
    }

    func get(index: UInt) -> Optional<T.SelfRef> {
         T.vecOfSelfGet(vecPtr: ptr, index: index)
    }

    /// Rust returns a UInt, but we cast to an Int because many Swift APIs such as
    /// `ForEach(0..rustVec.len())` expect Int.
    func len() -> Int {
        Int(T.vecOfSelfLen(vecPtr: ptr))
    }

    deinit {
        if isOwned {
            T.vecOfSelfFree(vecPtr: ptr)
        }
    }
}

extension RustVec: Sequence {
    func makeIterator() -> RustVecIterator<T> {
        return RustVecIterator(self)
    }
}

struct RustVecIterator<T: Vectorizable>: IteratorProtocol {
    var rustVec: RustVec<T>
    var index: UInt = 0

    init (_ rustVec: RustVec<T>) {
        self.rustVec = rustVec
    }

    mutating func next() -> T.SelfRef? {
        let val = rustVec.get(index: index)
        index += 1
        return val
    }
}

extension RustVec: Collection {
    typealias Index = Int

    func index(after i: Int) -> Int {
        i + 1
    }

    subscript(position: Int) -> T.SelfRef {
        self.get(index: UInt(position))!
    }

    var startIndex: Int {
        0
    }

    var endIndex: Int {
        self.len()
    }
}

extension RustVec: RandomAccessCollection {
}

extension UnsafeBufferPointer {
    func toFfiSlice () -> __private__FfiSlice {
        __private__FfiSlice(start: UnsafeMutablePointer(mutating: self.baseAddress), len: UInt(self.count))
    }
}

extension Array {
    /// Get an UnsafeBufferPointer to the array's content's first byte with the array's length.
    ///
    /// ```
    /// // BAD! Swift will immediately free the arrays memory and so your pointer is invalid.
    /// let pointer = useMyPointer([1, 2, 3].toUnsafeBufferPointer())
    ///
    /// // GOOD! The array will outlive the buffer pointer.
    /// let array = [1, 2, 3]
    /// useMyPointer(array.toUnsafeBufferPointer())
    /// ```
    func toUnsafeBufferPointer() -> UnsafeBufferPointer<Element> {
        UnsafeBufferPointer(start: UnsafePointer(self), count: self.count)
    }
}

protocol Vectorizable {
    associatedtype SelfRef
    associatedtype SelfRefMut

    static func vecOfSelfNew() -> UnsafeMutableRawPointer;

    static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer)

    static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Self)

    static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self>

    static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SelfRef>

    static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SelfRefMut>

    static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt
}
