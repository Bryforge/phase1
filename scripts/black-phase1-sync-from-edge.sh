#!/usr/bin/env sh
# Sync black-phase1 with latest edge/stable.
#
# Usage:
#   sh scripts/black-phase1-sync-from-edge.sh
#
# Purpose:
#   Keep the rapid test branch close to edge/stable while preserving local
#   safety. This script refuses dirty working trees and uses --force-with-lease
#   only on black-phase1, never on edge/stable.

set -eu

TEST_BRANCH="black-phase1"
EDGE_BRANCH="edge/stable"

fail() { printf 'black-phase1-sync-from-edge: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

current="$(git rev-parse --abbrev-ref HEAD)"
[ "$current" = "$TEST_BRANCH" ] || fail "expected branch $TEST_BRANCH, got $current"
[ -z "$(git status --short)" ] || fail "working tree is dirty; commit/stash first"

printf 'Syncing %s from %s\n' "$TEST_BRANCH" "$EDGE_BRANCH"

git fetch origin "$EDGE_BRANCH" "$TEST_BRANCH"
git rebase "origin/$EDGE_BRANCH"
git push --force-with-lease origin "$TEST_BRANCH"

printf '\nDONE: %s is rebased on origin/%s\n' "$TEST_BRANCH" "$EDGE_BRANCH"
git log --oneline -5
