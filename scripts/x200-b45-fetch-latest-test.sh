#!/usr/bin/env sh
# X200-side Phase1 fast fetch/build/prepare helper.
#
# Usage:
#   sh scripts/x200-b45-fetch-latest-test.sh /dev/sdb YES_WRITE_USB
#
# Purpose:
#   Pull the latest Mac/GitHub update, build on x86_64, preflight, then prepare
#   the verified B45 next-test USB.

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
BRANCH="edge/stable"

fail() { printf 'x200-b45-fetch-latest-test: %s\n' "$1" >&2; exit 1; }
[ -n "$USB" ] || fail "usage: sh scripts/x200-b45-fetch-latest-test.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from the phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"
command -v cargo >/dev/null 2>&1 || fail "missing cargo"
command -v file >/dev/null 2>&1 || fail "missing file"

if [ "$(id -u)" = "0" ]; then
  fail "do not run with sudo; wrappers call sudo internally"
fi

printf 'Phase1 X200 fetch latest test helper\n'
printf 'target: %s\n' "$USB"
printf 'branch: %s\n\n' "$BRANCH"

git fetch origin "$BRANCH"
git checkout "$BRANCH"
git pull --ff-only origin "$BRANCH"

git log -1 --oneline

cargo build --release
file target/release/phase1 | tee build/x200-b45-phase1-binary-file.txt
if ! file target/release/phase1 | grep -q 'x86-64'; then
  fail "target/release/phase1 is not x86_64; build final media on the X200/x86_64 builder"
fi

sh scripts/x200-b43-system-preflight.sh "$USB"
sh scripts/x200-b45-prepare-next-test.sh "$USB" YES_WRITE_USB

printf '\nDONE: X200 latest test media prepared.\n'
printf 'Expected final marker: RESULT: prepared_and_verified_for_next_test\n'
