#!/bin/bash

set -e

if [[ -z "$PROJECT_DIR" ]]; then
    echo "Missing PROJECT_DIR environment variable." 1>&2
    exit 1
fi

export PATH="$HOME/.cargo/bin:$PATH"

# Without this we can't compile on MacOS Big Sur
# https://github.com/TimNN/cargo-lipo/issues/41#issuecomment-774793892
if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]; then
  export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

cd "$PROJECT_DIR"

if [[ $CONFIGURATION == "Release" ]]; then
    echo "BUIlDING FOR RELEASE"
    
    cargo build --release --manifest-path ../crates/swift-integration-tests/Cargo.toml
else
    echo "BUIlDING FOR DEBUG"

    cargo build --manifest-path ../crates/swift-integration-tests/Cargo.toml
fi
