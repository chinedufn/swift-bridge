export SWIFT_BRIDGE_OUT_DIR=/Users/jonaseveraert/Documents/projects/swift-bridge-framework/swift-bridge/crates/swift-bridge-build/tests/sample_project/generated
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
