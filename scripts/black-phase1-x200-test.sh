#!/usr/bin/env sh
# black-phase1 X200 rapid test helper.
#
# Usage:
#   sh scripts/black-phase1-x200-test.sh /dev/sdb YES_WRITE_USB

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
BRANCH="black-phase1"

fail() { printf 'black-phase1-x200-test: %s\n' "$1" >&2; exit 1; }
[ -n "$USB" ] || fail "usage: sh scripts/black-phase1-x200-test.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"
command -v cargo >/dev/null 2>&1 || fail "missing cargo"
command -v file >/dev/null 2>&1 || fail "missing file"

if [ "$(id -u)" = "0" ]; then
  fail "do not run with sudo; wrappers call sudo internally"
fi

current="$(git rev-parse --abbrev-ref HEAD)"
[ "$current" = "$BRANCH" ] || fail "expected branch $BRANCH, got $current"

printf 'black-phase1 X200 rapid test\n'
printf 'target: %s\n\n' "$USB"

git fetch origin "$BRANCH"
git pull --ff-only origin "$BRANCH"
git log -1 --oneline

cargo build --release
file target/release/phase1
if ! file target/release/phase1 | grep -q 'x86-64'; then
  fail "target/release/phase1 is not x86_64"
fi

sh scripts/x200-b43-system-preflight.sh "$USB"
sh scripts/x200-b45-prepare-next-test.sh "$USB" YES_WRITE_USB

printf '\nDONE: black-phase1 X200 media prepared.\n'
printf 'Reboot only after RESULT: prepared_and_verified_for_next_test was printed.\n'
