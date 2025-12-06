#!/usr/bin/env bash
set -euo pipefail

# Run the CI steps locally for each libcamera version (same matrix as GitHub Actions).
VERSIONS=(
  v0.4.0
  v0.5.0
  v0.5.1
  v0.5.2
  v0.6.0
)

for ver in "${VERSIONS[@]}"; do
  echo "=== Building CI image for libcamera ${ver} ==="
  docker build -f docker/ci/Dockerfile --build-arg LIBCAMERA_VERSION="${ver}" -t "libcamera-ci:${ver}" .
  echo "=== Running CI steps for libcamera ${ver} ==="
  docker run --rm -v "$PWD:/workspace" -w /workspace "libcamera-ci:${ver}" ./docker/ci/entrypoint.sh
done
