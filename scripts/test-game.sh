#!/usr/bin/env sh
set -eu

cargo fmt --all -- --check
cargo test --bin phase1 arena
cargo test --bin phase1 wasm
cargo test --test game -- --nocapture
