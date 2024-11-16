# Multiple Bridge Modules

`swift-bridge` supports defining multiple bridge modules across one or more files.

This example demonstrates how to define and generate code for multiple bridge modules.

The Rust crate contains a `crate::bridge::user` and a `crate::bridge::bank` module.

Each module contains a bridge module that exposes types to `Swift`.

The `main.swift` function uses these bridged types to create and print some information.

## To Run

```sh
./run.sh
```
