#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

cargo build -p multiple-bridge-modules

swiftc -L ../../target/debug \
  -lmultiple_bridge_modules \
  -import-objc-header bridging-header.h \
  -framework CoreFoundation -framework SystemConfiguration \
  main.swift ./generated/SwiftBridgeCore.swift ./generated/multiple-bridge-modules/multiple-bridge-modules.swift

./main
