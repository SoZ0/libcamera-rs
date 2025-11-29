#!/usr/bin/env bash
set -euo pipefail

# Allow skipping doc build for speed/debugging.
: "${RUN_DOCS:=true}"

echo "Using libcamera from pkg-config:"
pkg-config --modversion libcamera || true

cargo build
cargo test
cargo clippy --no-deps -- -D warnings

if [ "$RUN_DOCS" = "true" ]; then
  RUSTDOCFLAGS="-Dwarnings" cargo doc --no-deps --lib
fi
