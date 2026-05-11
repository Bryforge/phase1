#!/usr/bin/env sh
# Base1 reorganization verification bundle.
#
# This script is read-only. It runs the current Base1 organization gates before
# any broader file movement is attempted.

set -eu

info() {
  printf 'base1-reorganization-verify: %s\n' "$1"
}

run() {
  printf '$ %s\n' "$*"
  "$@"
}

info 'mode: read-only'
info 'scope: Base1 organization readiness'

run sh scripts/base1-doc-integrity.sh
run sh scripts/base1-link-check.sh
run sh scripts/base1-test-inventory-verify.sh

if command -v cargo >/dev/null 2>&1; then
  run cargo test --all-targets
else
  info 'cargo not found; skipping cargo test --all-targets'
  info 'run cargo test --all-targets on a Rust-capable host before claiming full readiness'
fi

info 'verification complete; no files were changed'
