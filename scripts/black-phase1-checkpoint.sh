#!/usr/bin/env sh
# black-phase1 checkpoint helper.
#
# Usage:
#   sh scripts/black-phase1-checkpoint.sh
#   sh scripts/black-phase1-checkpoint.sh b45-minimal-cjk-before-test
#
# Purpose:
#   Create and push a checkpoint branch from the current black-phase1 HEAD
#   before risky boot/media tests.

set -eu

NAME="${1:-}"
TEST_BRANCH="black-phase1"
PREFIX="checkpoint/black-phase1"

fail() { printf 'black-phase1-checkpoint: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

current="$(git rev-parse --abbrev-ref HEAD)"
[ "$current" = "$TEST_BRANCH" ] || fail "expected branch $TEST_BRANCH, got $current"
[ -z "$(git status --short)" ] || fail "working tree is dirty; commit/stash before checkpoint"

git fetch origin "$TEST_BRANCH"
git pull --ff-only origin "$TEST_BRANCH"

stamp="$(date -u +%Y%m%d-%H%M%S)"
if [ -n "$NAME" ]; then
  safe_name="$(printf '%s' "$NAME" | tr '[:upper:] ' '[:lower:]-' | tr -cd 'a-z0-9._/-')"
  checkpoint="$PREFIX-$safe_name-$stamp"
else
  checkpoint="$PREFIX-$stamp"
fi

head_sha="$(git rev-parse HEAD)"

git branch "$checkpoint" "$head_sha"
git push origin "$checkpoint"

printf 'DONE: black-phase1 checkpoint pushed.\n'
printf 'branch: %s\n' "$checkpoint"
printf 'sha   : %s\n' "$head_sha"
printf '\nRestore later with:\n'
printf '  git fetch origin\n'
printf '  git checkout %s\n' "$checkpoint"
