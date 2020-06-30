#!/usr/bin/env bash
set -euo pipefail

#
# https://github.com/codecov/example-rust
#

exec docker run \
  --rm \
  --security-opt seccomp=unconfined \
  --volume "${PWD}:/volume" \
  xd009642/tarpaulin:latest-nightly \
  cargo tarpaulin \
    --all \
    --verbose \
    --exclude-files 'tests/*' \
    --exclude-files 'main.rs' \
    "$@"