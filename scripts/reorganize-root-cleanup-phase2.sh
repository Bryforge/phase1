#!/usr/bin/env sh
# Phase1 / Base1 / Fyr root cleanup phase 2.
#
# Default mode is dry-run.
#
# Usage:
#   sh scripts/reorganize-root-cleanup-phase2.sh --dry-run
#   sh scripts/reorganize-root-cleanup-phase2.sh --apply
#
# Purpose:
#   Move the remaining root-facing project notes, release docs, website files,
#   and loose legacy material into logical folders or junk preservation.

set -eu

MODE="${1:---dry-run}"
APPLY=0
PLAN_DIR="build/reorganization"
PLAN="$PLAN_DIR/root-cleanup-phase2-plan.txt"
REPORT="$PLAN_DIR/root-cleanup-phase2-report.env"

case "$MODE" in
  --dry-run|dry-run) APPLY=0 ;;
  --apply|apply) APPLY=1 ;;
  *) printf 'usage: sh scripts/reorganize-root-cleanup-phase2.sh --dry-run|--apply\n' >&2; exit 1 ;;
esac

fail() { printf 'reorganize-root-cleanup-phase2: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from repository root"
command -v git >/dev/null 2>&1 || fail "missing git"
mkdir -p "$PLAN_DIR"
: > "$PLAN"
: > "$REPORT"

log_plan() { printf '%s\n' "$*" >> "$PLAN"; }
mkdir_logged() {
  dir="$1"
  log_plan "mkdir -p $dir"
  if [ "$APPLY" -eq 1 ]; then mkdir -p "$dir"; else printf '[dry-run] mkdir -p %s\n' "$dir"; fi
}
move_if_exists() {
  src="$1"
  dst="$2"
  [ -e "$src" ] || return 0
  mkdir_logged "$(dirname "$dst")"
  log_plan "git mv $src $dst"
  if [ "$APPLY" -eq 1 ]; then git mv "$src" "$dst"; else printf '[dry-run] git mv %s %s\n' "$src" "$dst"; fi
}

printf 'Phase1/Base1/Fyr root cleanup phase 2\n'
printf 'mode: %s\n' "$MODE"
printf 'plan: %s\n\n' "$PLAN"

# Phase1 runtime/operator docs.
move_if_exists "OPERATOR_SHELL.md" "phase1/docs/OPERATOR_SHELL.md"
move_if_exists "TERMINAL.md" "phase1/docs/TERMINAL.md"
move_if_exists "LAUNCH.md" "phase1/docs/LAUNCH.md"
move_if_exists "DEV_DOCK.md" "phase1/docs/DEV_DOCK.md"
move_if_exists "LANGUAGE_RUNTIME.md" "phase1/docs/LANGUAGE_RUNTIME.md"
move_if_exists "STORAGE_GIT_RUST.md" "phase1/docs/STORAGE_GIT_RUST.md"
move_if_exists "PHASE1_EMACS.md" "phase1/docs/PHASE1_EMACS.md"
move_if_exists "SYSTEM_TEST_REPORT.md" "phase1/docs/SYSTEM_TEST_REPORT.md"
move_if_exists "CODE_REVIEW.md" "phase1/docs/CODE_REVIEW.md"
move_if_exists "QUALITY_AUDIT.md" "phase1/docs/QUALITY_AUDIT.md"
move_if_exists "QUALITY_SCORECARD.md" "phase1/docs/QUALITY_SCORECARD.md"

# Base1 hardware/release/readiness docs.
move_if_exists "RPI5_COMPAT.md" "base1/docs/targets/RPI5_COMPAT.md"
move_if_exists "RELEASE_BASE1_LIBREBOOT_READONLY_V1.md" "base1/docs/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md"
move_if_exists "RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md" "base1/docs/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md"
move_if_exists "RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md" "base1/docs/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md"
move_if_exists "RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md" "base1/docs/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md"
move_if_exists "RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md" "base1/docs/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md"
move_if_exists "RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md" "base1/docs/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md"
move_if_exists "DEVELOPMENT_CHECKPOINT_BASE1_RECOVERY_USB_READONLY_V1.md" "base1/docs/checkpoints/DEVELOPMENT_CHECKPOINT_BASE1_RECOVERY_USB_READONLY_V1.md"

# Fyr and future language/game experiments.
move_if_exists "GAME_DEV.md" "junk/experiments/game/GAME_DEV.md"
move_if_exists "GO_TESTING.md" "junk/experiments/go/GO_TESTING.md"
move_if_exists "AVIM.md" "junk/experiments/avim/AVIM.md"
move_if_exists "AVIM_PRO.md" "junk/experiments/avim/AVIM_PRO.md"

# AI/assistant notes preserved but not active root docs.
move_if_exists "AI_GINA.md" "junk/experiments/ai/AI_GINA.md"
move_if_exists "AI_GINA_ROADMAP.md" "junk/experiments/ai/AI_GINA_ROADMAP.md"

# Website/publicity/root site files.
move_if_exists "PUBLICITY.md" "docs/website/PUBLICITY.md"
move_if_exists "CONTACT.md" "docs/website/CONTACT.md"
move_if_exists "WIKI_ROADMAP.md" "docs/website/WIKI_ROADMAP.md"
move_if_exists "index.html" "root-site/index.html"
move_if_exists "links.html" "root-site/links.html"
move_if_exists "styles.css" "root-site/styles.css"
move_if_exists "site.js" "root-site/site.js"
move_if_exists "button-fix.css" "root-site/button-fix.css"
move_if_exists "button-fix.js" "root-site/button-fix.js"
move_if_exists "robots.txt" "root-site/robots.txt"
move_if_exists "sitemap.xml" "root-site/sitemap.xml"
move_if_exists "update_notes.txt" "junk/legacy/docs/update_notes.txt"
move_if_exists "DEV_NOTES.txt" "junk/legacy/docs/DEV_NOTES.txt"

# General releases/checkpoints/repo docs.
move_if_exists "CHANGELOG.md" "docs/releases/CHANGELOG.md"
move_if_exists "RELEASE_CHECKLIST.md" "docs/releases/RELEASE_CHECKLIST.md"
move_if_exists "RELEASE_NOTES_3.10.6.md" "docs/releases/RELEASE_NOTES_3.10.6.md"
move_if_exists "RELEASE_NOTES_v3.6.0.md" "docs/releases/RELEASE_NOTES_v3.6.0.md"
move_if_exists "RELEASE_v4.0.0.md" "docs/releases/RELEASE_v4.0.0.md"
move_if_exists "RELEASE_v4.1.0.md" "docs/releases/RELEASE_v4.1.0.md"
move_if_exists "RELEASE_v4.2.0.md" "docs/releases/RELEASE_v4.2.0.md"
move_if_exists "RELEASE_v4.3.0.md" "docs/releases/RELEASE_v4.3.0.md"
move_if_exists "RELEASE_v4.4.0.md" "docs/releases/RELEASE_v4.4.0.md"
move_if_exists "RELEASE_v5.0.0.md" "docs/releases/RELEASE_v5.0.0.md"
move_if_exists "NEXT_UPDATE_v3.7.0.md" "junk/legacy/docs/updates/NEXT_UPDATE_v3.7.0.md"
move_if_exists "NEXT_UPDATE_v3.8.0.md" "junk/legacy/docs/updates/NEXT_UPDATE_v3.8.0.md"
move_if_exists "DEVELOPMENT_CHECKPOINT_EDGE_4_3_0_DEV.md" "junk/legacy/docs/checkpoints/DEVELOPMENT_CHECKPOINT_EDGE_4_3_0_DEV.md"
move_if_exists "DEVELOPMENT_CHECKPOINT_EDGE_6_0_0.md" "junk/legacy/docs/checkpoints/DEVELOPMENT_CHECKPOINT_EDGE_6_0_0.md"
move_if_exists "DEVELOPMENT_CHECKPOINT_LEARN.md" "junk/legacy/docs/checkpoints/DEVELOPMENT_CHECKPOINT_LEARN.md"
move_if_exists "CHECKPOINT_REPO_DOCTRINE_EDGE_STABLE.md" "junk/legacy/docs/checkpoints/CHECKPOINT_REPO_DOCTRINE_EDGE_STABLE.md"
move_if_exists "DOCS_SYNC_CHECKPOINT.md" "junk/legacy/docs/checkpoints/DOCS_SYNC_CHECKPOINT.md"
move_if_exists "EDGE_STABLE_CHECKPOINT.md" "junk/legacy/docs/checkpoints/EDGE_STABLE_CHECKPOINT.md"
move_if_exists "REPO_CHANNELS.md" "shared/docs/repo/REPO_CHANNELS.md"
move_if_exists "REPO_DOCTRINE.md" "shared/docs/repo/REPO_DOCTRINE.md"
move_if_exists "ROADMAP_DESIGNS.md" "shared/docs/roadmaps/ROADMAP_DESIGNS.md"
move_if_exists "UPDATE_PROTOCOL.md" "shared/docs/UPDATE_PROTOCOL.md"

# Root helper launchers/one-off scripts.
move_if_exists "run-phase1-uefi.sh" "scripts/launchers/run-phase1-uefi.sh"
move_if_exists "start_phase1" "scripts/launchers/start_phase1"

cat > "$REPORT" <<EOF
PHASE1_ROOT_CLEANUP_MODE=$MODE
PHASE1_ROOT_CLEANUP_APPLY=$APPLY
PHASE1_ROOT_CLEANUP_PLAN=$PLAN
PHASE1_ROOT_CLEANUP_RESULT=$([ "$APPLY" -eq 1 ] && printf applied || printf dry_run)
EOF

printf '\nRoot cleanup phase 2 %s complete.\n' "$([ "$APPLY" -eq 1 ] && printf applied || printf dry-run)"
printf 'Plan: %s\n' "$PLAN"
printf 'Report: %s\n' "$REPORT"
if [ "$APPLY" -eq 0 ]; then
  printf '\nReview the plan, then run:\n'
  printf '  sh scripts/reorganize-root-cleanup-phase2.sh --apply\n'
else
  printf '\nNext checks:\n'
  printf '  git status --short\n'
  printf '  find . -maxdepth 1 -type f | sort\n'
fi
