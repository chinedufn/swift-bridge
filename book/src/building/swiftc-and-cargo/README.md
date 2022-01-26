# swiftc + Cargo

## Swift links to a Rust native library

... TODO ... demonstrate compiling a Rust native library using Cargo
and then using swiftc to link the native library into the final Swift binary.

```
# TODO: Flesh out an example around something like this...

SWIFT_BRIDGE_OUT_DIR=$OUTDIR cargo build --target x86_64-apple-darwin

swiftc -L target/x86_64-apple-darwin/debug/ -lswiftc_link_rust -import-objc-header bridging-header.h \
  main.swift lib.swift ./generated/swiftc-and-cargo/swiftc-and-cargo.swift
```

## Rust links to a Swift native library

... TODO ... demonstrate compiling a Swift native library using swiftc
and then using Cargo to link the native library into the final Rust binary.

```
# TODO: Flesh out an example around something like this...

swiftc -emit-library -static -module-name my_swift -import-objc-header bridging-header.h \
  lib.swift ./generated/swiftc-and-cargo/swiftc-and-cargo.swift
```
