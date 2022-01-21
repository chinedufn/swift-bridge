#!/bin/bash

# Integration tests between Swift and Rust

set -e

export RUSTFLAGS="-D warnings"

# Change to the root directory of the Xcode project
THIS_DIR=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
ROOT_DIR="$THIS_DIR"
cd $ROOT_DIR

cd SwiftRustIntegrationTestRunner

xcodebuild \
  -project SwiftRustIntegrationTestRunner.xcodeproj \
  -scheme SwiftRustIntegrationTestRunner \
  clean test
