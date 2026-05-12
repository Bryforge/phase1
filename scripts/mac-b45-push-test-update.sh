#!/usr/bin/env sh
# Mac-side Phase1 fast update helper.
#
# Usage:
#   sh scripts/mac-b45-push-test-update.sh "describe the update"
#
# Purpose:
#   Commit local Mac edits, rebase onto origin/edge/stable, and push quickly.
#   This does not build final X200 boot media.

set -eu

MSG="${1:-}"
BRANCH="edge/stable"

fail() { printf 'mac-b45-push-test-update: %s\n' "$1" >&2; exit 1; }
[ -n "$MSG" ] || fail "usage: sh scripts/mac-b45-push-test-update.sh \"commit message\""
[ -d .git ] || fail "run from the phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

current="$(git rev-parse --abbrev-ref HEAD)"
[ "$current" = "$BRANCH" ] || fail "expected branch $BRANCH, got $current"

printf 'Phase1 Mac push helper\n'
printf 'branch : %s\n' "$current"
printf 'message: %s\n\n' "$MSG"

printf '%s\n' '--- status before ---'
git status --short

if [ -z "$(git status --short)" ]; then
  fail "no local changes to commit"
fi

printf '%s\n' '--- basic secret/IP scan ---'
if git diff -- . ':!target' ':!build' | grep -E '192\.168\.|10\.[0-9]+\.|172\.(1[6-9]|2[0-9]|3[0-1])\.|token\.txt|PRIVATE KEY|BEGIN OPENSSH|ghp_|github_pat_' >/tmp/phase1-mac-secret-scan.txt 2>/dev/null; then
  cat /tmp/phase1-mac-secret-scan.txt
  fail "possible private IP/token/key material found in diff; clean it before push"
fi
printf 'scan: pass\n'

git add -A
git commit -m "$MSG"

git fetch origin "$BRANCH"
git rebase "origin/$BRANCH"
git push origin "$BRANCH"

printf '\nDONE: pushed Mac update to origin/%s\n' "$BRANCH"
git log -1 --oneline
