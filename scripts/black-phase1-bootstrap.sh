#!/usr/bin/env sh
# black-phase1 bootstrap helper.
#
# Usage:
#   sh scripts/black-phase1-bootstrap.sh
#   sh scripts/black-phase1-bootstrap.sh /dev/sdb
#
# Purpose:
#   Get any local clone onto the rapid test branch, pull the latest branch
#   state, and run the black-phase1 doctor. This performs no disk writes.

set -eu

TARGET="${1:-}"
BRANCH="black-phase1"
EDGE="edge/stable"

fail() { printf 'black-phase1-bootstrap: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

printf 'black-phase1 bootstrap\n'
printf 'target branch: %s\n' "$BRANCH"
printf 'edge branch  : %s\n' "$EDGE"
[ -n "$TARGET" ] && printf 'usb target   : %s\n' "$TARGET"
printf '\n'

if [ -n "$(git status --short)" ]; then
  printf 'Working tree has local changes. Commit/stash before branch bootstrap.\n' >&2
  git status --short >&2
  exit 1
fi

git fetch origin "$BRANCH" "$EDGE"

if git show-ref --verify --quiet "refs/heads/$BRANCH"; then
  git checkout "$BRANCH"
else
  git checkout -b "$BRANCH" "origin/$BRANCH"
fi

git pull --ff-only origin "$BRANCH"

printf '\nCurrent branch state:\n'
git log -1 --oneline
printf '\nRecent black-phase1 commits:\n'
git log --oneline -8

if [ -f scripts/black-phase1-doctor.sh ]; then
  if [ -n "$TARGET" ]; then
    sh scripts/black-phase1-doctor.sh "$TARGET"
  else
    sh scripts/black-phase1-doctor.sh
  fi
else
  printf '\nDoctor script not present yet. Bootstrap complete.\n'
fi
