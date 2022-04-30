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

# If project files don't exist before Xcode begins building we get something like:
# error: Build input file cannot be found: '/path/to/Generated/SwiftBridgeCore.swift'
# So.. here we create empty versions of the files that will get generated during the
# build so that Xcode knows about them.
# During the build process these will get overwritten with their real final contents.
touch ./Generated/SwiftBridgeCore.{h,swift}
mkdir -p ./Generated/swift-integration-tests
touch ./Generated/swift-integration-tests/swift-integration-tests.{h,swift}

xcodebuild \
  -project SwiftRustIntegrationTestRunner.xcodeproj \
  -scheme SwiftRustIntegrationTestRunner \
  clean test
