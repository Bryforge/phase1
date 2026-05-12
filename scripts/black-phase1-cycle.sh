#!/usr/bin/env sh
# black-phase1 full rapid test cycle helper.
#
# Usage:
#   sh scripts/black-phase1-cycle.sh /dev/sdb YES_WRITE_USB "checkpoint-name"
#
# Purpose:
#   Run the full rapid branch cycle before a hardware test:
#   - ensure branch is black-phase1;
#   - pull latest black-phase1;
#   - run status;
#   - run doctor;
#   - create a checkpoint;
#   - prepare verified X200 test media.
#
# Do not run this whole wrapper with sudo. Called scripts use sudo internally
# when media writing is required.

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
CHECKPOINT_NAME="${3:-pre-test}"
BRANCH="black-phase1"

fail() { printf 'black-phase1-cycle: %s\n' "$1" >&2; exit 1; }
[ -n "$USB" ] || fail "usage: sh scripts/black-phase1-cycle.sh /dev/sdb YES_WRITE_USB checkpoint-name"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

if [ "$(id -u)" = "0" ]; then
  fail "do not run with sudo; wrappers call sudo internally"
fi

current="$(git rev-parse --abbrev-ref HEAD)"
[ "$current" = "$BRANCH" ] || fail "expected branch $BRANCH, got $current"

if [ -n "$(git status --short)" ]; then
  printf 'Working tree has local changes. Commit or stash before full cycle.\n' >&2
  git status --short >&2
  exit 1
fi

printf 'black-phase1 full test cycle\n'
printf 'target    : %s\n' "$USB"
printf 'checkpoint: %s\n\n' "$CHECKPOINT_NAME"

git fetch origin "$BRANCH"
git pull --ff-only origin "$BRANCH"

git log -1 --oneline

sh scripts/black-phase1-status-report.sh
sh scripts/black-phase1-doctor.sh "$USB"
sh scripts/black-phase1-checkpoint.sh "$CHECKPOINT_NAME"
sh scripts/black-phase1-x200-test.sh "$USB" YES_WRITE_USB

printf '\nDONE: black-phase1 full cycle completed.\n'
printf 'Reboot only if the media prep printed:\n'
printf '  RESULT: prepared_and_verified_for_next_test\n'
