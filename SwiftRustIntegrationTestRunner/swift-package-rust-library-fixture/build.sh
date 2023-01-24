#!/usr/bin/env bash

THIS_DIR=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
ROOT_DIR="$THIS_DIR"
cd $ROOT_DIR

cargo build --target x86_64-apple-darwin --target-dir "$(pwd)/target"
cargo build --target aarch64-apple-darwin --target-dir "$(pwd)/target"

mkdir -p "$(pwd)/target/universal/"

lipo \
    $(pwd)/target/aarch64-apple-darwin/debug/libtest_swift_packages.a \
    $(pwd)/target/x86_64-apple-darwin/debug/libtest_swift_packages.a -create -output \
    $(pwd)/target/universal/libtest_swift_packages.a

