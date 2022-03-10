#!/usr/bin/env bash

THIS_DIR=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
ROOT_DIR="$THIS_DIR"
cd $ROOT_DIR

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"
cargo build --target x86_64-apple-darwin --target-dir "$(pwd)/target"
