#!/usr/bin/env bash

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"
cargo build --target x86_64-apple-darwin --target-dir "$(pwd)/target"
cargo build --target aarch64-apple-ios --target-dir "$(pwd)/target"
cargo build --target x86_64-apple-ios --target-dir "$(pwd)/target"
