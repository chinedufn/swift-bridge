#!/bin/bash

# Swift package tests

set -e

export RUSTFLAGS="-D warnings"

# Change to the root directory of the Xcode project
cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P
THIS_DIR=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
ROOT_DIR="$THIS_DIR"
cd $ROOT_DIR

# Make a temp directory
TEMP_DIR=$(mktemp -d)

# Delete the temp directory before the shell exits
trap 'rm -rf $TEMP_DIR' EXIT

# Copy directories related to all of the building and test running to the temp directory
for DIR in crates src examples SwiftRustIntegrationTestRunner
do
  cp -r $DIR/ $TEMP_DIR/$DIR 
done
cp Cargo.toml $TEMP_DIR
cd $TEMP_DIR/SwiftRustIntegrationTestRunner

# Build Rust
mkdir -p swift-package-rust-library-fixture/generated

./swift-package-rust-library-fixture/build.sh

# Create Swift Package
cargo run -p integration-test-create-swift-package

# Test Swift Package
cd swift-package-test-package
swift test
