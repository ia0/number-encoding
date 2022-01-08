#!/bin/sh
set -e

info() {
  echo "[1;36m$1[m"
}

info_exec() {
  info "$*"
  "$@"
}

for toolchain in stable nightly; do
  cargo_build="info_exec cargo +$toolchain build --release"
  $cargo_build
  $cargo_build --no-default-features --features=alloc
  $cargo_build --no-default-features
done
info_exec cargo test
info_exec cargo fmt -- --check
info_exec cargo clippy -- --deny=warnings

info "Done"
