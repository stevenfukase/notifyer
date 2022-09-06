#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf

cargo build --release --target=${TARGET_ARCH}
