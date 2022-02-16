#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"

cargo build -p async-functions

swiftc -L ../../target/debug \
  -lasync_functions \
  -import-objc-header bridging-header.h \
  main.swift ./generated/SwiftBridgeCore.swift ./generated/async-functions/async-functions.swift
