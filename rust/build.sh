#!/bin/bash
set -e

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"

cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-ios 
cargo build --target x86_64-apple-ios