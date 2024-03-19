#!/bin/bash

PACKAGE_NAME=test-release-succeeds
LIBRARY_NAME=test_release_succeeds
SWIFT_PACKAGE_NAME=TestReleaseSucceeds
EXECUTABLE_TARGET_NAME="$SWIFT_PACKAGE_NAME"Runner

set -e

THISDIR=$(dirname $0)
cd $THISDIR

echo "Building for macOS..."
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --target x86_64-apple-darwin --release
cargo build --target aarch64-apple-darwin --release
mkdir -p ./target/universal-macos/release
lipo \
    ./target/aarch64-apple-darwin/release/lib$LIBRARY_NAME.a \
    ./target/x86_64-apple-darwin/release/lib$LIBRARY_NAME.a -create -output \
    ./target/universal-macos/release/lib$LIBRARY_NAME.a

function create_package {
  swift-bridge-cli create-package \
        --bridges-dir ./generated \
        --out-dir $SWIFT_PACKAGE_NAME \
        --ios ./target/universal-macos/release/lib$LIBRARY_NAME.a \
        --name $SWIFT_PACKAGE_NAME
}

function patch_package {
    cp Package.swift $SWIFT_PACKAGE_NAME/Package.swift
    mkdir -p $SWIFT_PACKAGE_NAME/Sources/$EXECUTABLE_TARGET_NAME
    cp main.swift $SWIFT_PACKAGE_NAME/Sources/$EXECUTABLE_TARGET_NAME/main.swift
    cp SwiftExterns.swift $SWIFT_PACKAGE_NAME/Sources/$SWIFT_PACKAGE_NAME/SwiftExterns.swift
}

echo "Creating Swift package..."
create_package
patch_package

echo "Building for release..."
cd $SWIFT_PACKAGE_NAME
xcodebuild archive -scheme $EXECUTABLE_TARGET_NAME -archivePath ./build/$EXECUTABLE_TARGET_NAME.xcarchive -destination "platform=macOS"