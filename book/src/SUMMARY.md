# Summary

[Introduction](./README.md)

--- 

- [Building](./building/README.md)
  - [Xcode + Cargo](./building/xcode-and-cargo/README.md)
  - [swiftc + Cargo](./building/swiftc-and-cargo/README.md)
  - [Swift Package](./building/swift-package/README.md)

- [The Bridge Module](./bridge-module/README.md)
  - [extern "Rust"](./bridge-module/extern-rust/README.md)
  - [extern "Swift"](./bridge-module/extern-swift/README.md)
  - [Shared Structs](./bridge-module/shared-structs/README.md)
  - [Shared Enums](./bridge-module/shared-enums/README.md)
  - [Conditional Compilation](./bridge-module/conditional-compilation/README.md)

- [Built In Types](./built-in/README.md)
  - [Option<T> <---> Optional<T>](./built-in/option/README.md)
  - [Vec<T> <---> RustVec<T>](./built-in/vec/README.md)
  - [String <---> String](./built-in/string/README.md)
  - [&str <---> RustStr](./built-in/str/README.md)

- [Internal Design](./internal-design/README.md)
  - [Code Generation](./internal-design/codegen/README.md)
