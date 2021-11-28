#!/bin/bash

##################################################
# We call this from an Xcode run script.
##################################################

set -e

export PATH="$HOME/.cargo/bin:$PATH"

export SWIFT_BRIDGE_OUT_DIR="${PROJECT_DIR}/Generated"

# if [ $ENABLE_PREVIEWS == "NO" ]; then

  if [[ $CONFIGURATION == "Release" ]]; then
      echo "BUIlDING FOR RELEASE"
      
      cargo build --release --manifest-path ../Cargo.toml
  else
      echo "BUIlDING FOR DEBUG"

      cargo build --manifest-path ../Cargo.toml
  fi
  
# else
#   echo "Skipping the script because of preview mode"
# fi
