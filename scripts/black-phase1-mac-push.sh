#!/usr/bin/env sh
# black-phase1 Mac rapid push helper.
#
# Usage:
#   sh scripts/black-phase1-mac-push.sh "Describe rapid test"

set -eu

MSG="${1:-}"
BRANCH="black-phase1"

fail() { printf 'black-phase1-mac-push: %s\n' "$1" >&2; exit 1; }
[ -n "$MSG" ] || fail "usage: sh scripts/black-phase1-mac-push.sh \"commit message\""
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

current="$(git rev-parse --abbrev-ref HEAD)"
[ "$current" = "$BRANCH" ] || fail "expected branch $BRANCH, got $current"

printf 'black-phase1 Mac rapid push\n'
printf 'message: %s\n\n' "$MSG"

git status --short
[ -n "$(git status --short)" ] || fail "no local changes to commit"

printf '%s\n' '--- safety scan ---'
if git diff -- . ':!target' ':!build' | grep -E 'token\.txt|PRIVATE KEY|BEGIN OPENSSH|ghp_|github_pat_|AKIA|SECRET_ACCESS_KEY' >/tmp/black-phase1-secret-scan.txt 2>/dev/null; then
  cat /tmp/black-phase1-secret-scan.txt
  fail "possible token/key material found in diff"
fi
printf 'scan: pass\n'

git add -A
git commit -m "$MSG"
git fetch origin "$BRANCH"
git rebase "origin/$BRANCH" || {
  printf 'rebase failed; resolve manually, then run git rebase --continue\n' >&2
  exit 1
}
git push origin "$BRANCH"

printf '\nDONE: pushed to origin/%s\n' "$BRANCH"
git log -1 --oneline
