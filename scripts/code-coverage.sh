#!/usr/bin/env bash
set -euo pipefail

#
# https://github.com/codecov/example-rust
#

wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz &&
  tar xzf master.tar.gz &&
  cd kcov-master &&
  mkdir build &&
  cd build &&
  cmake .. &&
  make &&
  make install DESTDIR=../../kcov-build &&
  cd ../.. &&
  rm -rf kcov-master &&
  for file in target/debug/influxdb_client_rust-*[^\.d]; do [ -x "${file}" ] || continue; mkdir -p "target/cov/$(basename $file)"; ./kcov-build/usr/local/bin/kcov-system-daemon --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"; done &&
  bash <(curl -s https://codecov.io/bash) &&
  echo "Uploaded code coverage"