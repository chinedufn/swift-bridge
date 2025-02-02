#!/bin/bash -e

export RUSTFLAGS="-D warnings"

# Change to the root directory of the Xcode project
THIS_DIR=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
ROOT_DIR="$THIS_DIR"
cd "$ROOT_DIR"

# Compile the rust library used by the integration tests.
cargo build --manifest-path crates/swift-integration-tests/Cargo.toml

# Create a swift file with the generated swift code and an import for the
# generated C code. This is necessary because SPM projects don't support
# mixed-source - the C code has to be a separate SPM target.
OUT_DIR="integration-tests/Sources/Generated"
DST="integration-tests/Sources/SharedLib/Generated/SharedLib.swift"
echo "import RustLib" > "$DST"
cat "${OUT_DIR}/SwiftBridgeCore.swift" >> "$DST"
cat "${OUT_DIR}/swift-integration-tests/swift-integration-tests.swift" >> "$DST"

# Run the swift package tests. Note that we have to instruct swift where to look
# for the rust static lib.
(cd integration-tests && swift test)

