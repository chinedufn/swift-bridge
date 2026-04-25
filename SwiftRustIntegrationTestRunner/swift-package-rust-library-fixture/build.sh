#!/usr/bin/env bash

THIS_DIR=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
ROOT_DIR="$THIS_DIR"
cd $ROOT_DIR

RUST_TARGET="${RUST_TARGET:-aarch64-apple-darwin}"

cargo build --target "$RUST_TARGET" --target-dir "$(pwd)/target"

mkdir -p "$(pwd)/target/universal/"

cp \
    "$(pwd)/target/$RUST_TARGET/debug/libtest_swift_packages.a" \
    "$(pwd)/target/universal/libtest_swift_packages.a"
