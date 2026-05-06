#!/usr/bin/env sh
set -eu

# Phase1 runtime launcher
# Enables trusted host runtimes from the start so Python, Rust/lang, C, plugins,
# and other guarded language tools work without manually pressing 4 then t.
# This is intentionally explicit: host tools can run local programs.

cd "$(dirname "$0")/.."

export PHASE1_SAFE_MODE=0
export PHASE1_ALLOW_HOST_TOOLS=1
export PHASE1_BLEEDING_EDGE="${PHASE1_BLEEDING_EDGE:-1}"
export PHASE1_DEVICE_MODE="${PHASE1_DEVICE_MODE:-laptop}"

echo "phase1 runtimes: SHIELD off + TRUST HOST on for local language runtimes"
echo "phase1 runtimes: Python/Rust/lang/C/plugins may execute host tools"

exec cargo run --bin phase1 -- "$@"
