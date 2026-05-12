#!/usr/bin/env sh
# black-phase1 status report helper.
#
# Usage:
#   sh scripts/black-phase1-status-report.sh
#
# Purpose:
#   Print a compact status report for the rapid branch, including commits that
#   are on black-phase1 but not yet on edge/stable. No writes are performed.

set -eu

TEST_BRANCH="black-phase1"
EDGE_BRANCH="edge/stable"
OUT_DIR="build/black-phase1-status"
REPORT="$OUT_DIR/black-phase1-status.env"

fail() { printf 'black-phase1-status-report: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"
mkdir -p "$OUT_DIR"
: > "$REPORT"

kv() { printf '%s=%s\n' "$1" "$2" | tee -a "$REPORT" >/dev/null; }

current="$(git rev-parse --abbrev-ref HEAD)"
head="$(git log -1 --oneline 2>/dev/null || echo unknown)"
status="$(git status --short 2>/dev/null || true)"

git fetch origin "$TEST_BRANCH" "$EDGE_BRANCH" >/dev/null 2>&1 || true

black_head="$(git rev-parse --short "origin/$TEST_BRANCH" 2>/dev/null || echo unknown)"
edge_head="$(git rev-parse --short "origin/$EDGE_BRANCH" 2>/dev/null || echo unknown)"
counts="$(git rev-list --left-right --count "origin/$EDGE_BRANCH...origin/$TEST_BRANCH" 2>/dev/null || echo 'unknown unknown')"
behind_edge="$(printf '%s' "$counts" | awk '{print $1}')"
ahead_edge="$(printf '%s' "$counts" | awk '{print $2}')"

kv BLACK_PHASE1_STATUS_CURRENT_BRANCH "$current"
kv BLACK_PHASE1_STATUS_LOCAL_HEAD "$head"
kv BLACK_PHASE1_STATUS_ORIGIN_BLACK_HEAD "$black_head"
kv BLACK_PHASE1_STATUS_ORIGIN_EDGE_HEAD "$edge_head"
kv BLACK_PHASE1_STATUS_BLACK_BEHIND_EDGE "$behind_edge"
kv BLACK_PHASE1_STATUS_BLACK_AHEAD_EDGE "$ahead_edge"
kv BLACK_PHASE1_STATUS_WORKTREE_DIRTY "$([ -n "$status" ] && printf yes || printf no)"

printf 'black-phase1 status report\n'
printf 'current branch : %s\n' "$current"
printf 'local head     : %s\n' "$head"
printf 'origin black   : %s\n' "$black_head"
printf 'origin edge    : %s\n' "$edge_head"
printf 'behind edge    : %s\n' "$behind_edge"
printf 'ahead edge     : %s\n' "$ahead_edge"
printf 'dirty tree     : %s\n' "$([ -n "$status" ] && printf yes || printf no)"
printf 'report         : %s\n\n' "$REPORT"

if [ -n "$status" ]; then
  printf 'local changes:\n'
  git status --short
  printf '\n'
fi

printf 'commits on black-phase1 not on edge/stable:\n'
if git rev-parse "origin/$EDGE_BRANCH" >/dev/null 2>&1 && git rev-parse "origin/$TEST_BRANCH" >/dev/null 2>&1; then
  git log --oneline "origin/$EDGE_BRANCH..origin/$TEST_BRANCH" || true
else
  printf 'unavailable\n'
fi

printf '\nrecommended next commands:\n'
if [ "$current" != "$TEST_BRANCH" ]; then
  printf '  git checkout %s\n' "$TEST_BRANCH"
elif [ -n "$status" ]; then
  printf '  sh scripts/black-phase1-mac-push.sh "Describe rapid test"\n'
elif [ "$behind_edge" != "0" ] && [ "$behind_edge" != "unknown" ]; then
  printf '  sh scripts/black-phase1-sync-from-edge.sh\n'
else
  printf '  sh scripts/black-phase1-doctor.sh /dev/sdb\n'
  printf '  sh scripts/black-phase1-x200-test.sh /dev/sdb YES_WRITE_USB\n'
fi
