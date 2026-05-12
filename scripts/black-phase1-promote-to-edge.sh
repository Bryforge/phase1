#!/usr/bin/env sh
# Promote known-good black-phase1 work to edge/stable.
#
# Usage from a clean local repo:
#   sh scripts/black-phase1-promote-to-edge.sh <commit-or-range> "Promotion message"
#
# Examples:
#   sh scripts/black-phase1-promote-to-edge.sh abc1234 "Promote B45 minimal glyph fix"
#   sh scripts/black-phase1-promote-to-edge.sh abc1234..def5678 "Promote B45 renderer path"
#
# This script intentionally refuses dirty working trees and never force-pushes
# edge/stable.

set -eu

RANGE="${1:-}"
MSG="${2:-}"
EDGE="edge/stable"
TEST="black-phase1"

fail() { printf 'black-phase1-promote-to-edge: %s\n' "$1" >&2; exit 1; }
[ -n "$RANGE" ] || fail "usage: sh scripts/black-phase1-promote-to-edge.sh <commit-or-range> \"message\""
[ -n "$MSG" ] || fail "missing promotion message"
[ -d .git ] || fail "run from phase1 repository root"
command -v git >/dev/null 2>&1 || fail "missing git"

[ -z "$(git status --short)" ] || fail "working tree is dirty; commit/stash first"

printf 'black-phase1 promotion helper\n'
printf 'range/message: %s / %s\n\n' "$RANGE" "$MSG"

git fetch origin "$EDGE" "$TEST"

git checkout "$EDGE"
git pull --ff-only origin "$EDGE"

case "$RANGE" in
  *..*) git cherry-pick "$RANGE" ;;
  *) git cherry-pick "$RANGE" ;;
esac

# Record promotion metadata without rewriting the promoted commit content.
mkdir -p build/black-phase1-promotion
cat > build/black-phase1-promotion/last-promotion.env <<EOF
BLACK_PHASE1_PROMOTION_RANGE=$RANGE
BLACK_PHASE1_PROMOTION_MESSAGE=$MSG
BLACK_PHASE1_PROMOTION_TARGET=$EDGE
BLACK_PHASE1_PROMOTION_SOURCE=$TEST
BLACK_PHASE1_PROMOTION_HEAD=$(git rev-parse --short HEAD)
EOF

printf '\nPromotion cherry-pick complete. Run validation before push:\n'
printf '  cargo build --release\n'
printf '  sh scripts/x200-b43-system-preflight.sh /dev/sdb\n'
printf '\nThen push if clean:\n'
printf '  git push origin %s\n' "$EDGE"
