#!/usr/bin/env bash
set -euo pipefail

# Allow skipping doc build for speed/debugging.
: "${RUN_DOCS:=true}"

LIBCAMERA_VERSION=$(pkg-config --modversion libcamera || true)
echo "Using libcamera from pkg-config: ${LIBCAMERA_VERSION}"

# Keep build artifacts separate per libcamera version to avoid cross-version cache issues.
if [ -z "${CARGO_TARGET_DIR:-}" ] && [ -n "${LIBCAMERA_VERSION}" ]; then
  export CARGO_TARGET_DIR="target/${LIBCAMERA_VERSION}"
fi
echo "Using CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-target}"

cargo build
cargo test
cargo clippy --no-deps -- -D warnings

if [ "$RUN_DOCS" = "true" ]; then
  RUSTDOCFLAGS="-Dwarnings" cargo doc --no-deps --lib
fi
