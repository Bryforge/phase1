#!/usr/bin/env sh
# black-phase1 worktree setup helper.
#
# Usage from an existing phase1 clone:
#   sh scripts/black-phase1-worktree-setup.sh
#   sh scripts/black-phase1-worktree-setup.sh ../phase1-black
#
# Purpose:
#   Create a separate black-phase1 working directory so rapid experiments do
#   not disturb the main edge/stable checkout.

set -eu

DEST="${1:-../phase1-black}"
BRANCH="black-phase1"
EDGE="edge/stable"

fail() { printf 'black-phase1-worktree-setup: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from an existing phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

printf 'black-phase1 worktree setup\n'
printf 'destination: %s\n' "$DEST"
printf 'branch     : %s\n\n' "$BRANCH"

if [ -n "$(git status --short)" ]; then
  printf 'Current checkout has local changes. Worktree setup is still safe, but review them first:\n'
  git status --short
  printf '\n'
fi

git fetch origin "$BRANCH" "$EDGE"

if git worktree list | awk '{print $1}' | grep -Fx "$DEST" >/dev/null 2>&1; then
  printf 'worktree already exists: %s\n' "$DEST"
else
  git worktree add "$DEST" "origin/$BRANCH"
fi

cd "$DEST"
# Make the local branch track origin/black-phase1 when possible.
current="$(git rev-parse --abbrev-ref HEAD)"
if [ "$current" = "HEAD" ]; then
  git checkout -B "$BRANCH" "origin/$BRANCH"
else
  git checkout "$BRANCH" 2>/dev/null || git checkout -B "$BRANCH" "origin/$BRANCH"
fi
git pull --ff-only origin "$BRANCH"

printf '\nDONE: black-phase1 worktree ready.\n'
printf 'Path: %s\n' "$(pwd)"
printf '\nNext commands:\n'
printf '  cd %s\n' "$(pwd)"
printf '  sh scripts/black-phase1.sh status\n'
printf '  sh scripts/black-phase1.sh doctor\n'
