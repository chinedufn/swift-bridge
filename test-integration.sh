#!/bin/bash

# Integration tests between Swift and Rust

set -e

export RUSTFLAGS="-D warnings"

# Change to the root directory of the Xcode project
THIS_DIR=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
ROOT_DIR="$THIS_DIR"
cd $ROOT_DIR

cd SwiftRustIntegrationTestRunner

# Delete previous generated files/folders
rm -r swift-package-rust-library-fixture/generated || true
rm -r swift-package-rust-library-fixture/MySwiftPackage || true
rm -r swift-package-rust-library-fixture/target || true
rm -r swift-package-test-package/.build || true

# Build Rust
mkdir swift-package-rust-library-fixture/generated

./swift-package-rust-library-fixture/build.sh

# Create Swift Package
cargo run -p integration-test-create-swift-package

# Test Swift Package
cd swift-package-test-package
swift test
cd ..


xcodebuild \
  -project SwiftRustIntegrationTestRunner.xcodeproj \
  -scheme SwiftRustIntegrationTestRunner \
  clean test
