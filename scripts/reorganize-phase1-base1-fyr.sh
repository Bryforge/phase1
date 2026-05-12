#!/usr/bin/env sh
# Phase1 / Base1 / Fyr repository reorganization helper.
#
# Default mode is dry-run.
#
# Usage:
#   sh scripts/reorganize-phase1-base1-fyr.sh --dry-run
#   sh scripts/reorganize-phase1-base1-fyr.sh --apply
#
# This first pass is intentionally conservative. It creates the new structure
# and moves only obvious backup/root-facing/historical files. It does not move
# Cargo src/, tests/, or .github in this pass.

set -eu

MODE="${1:---dry-run}"
PLAN_DIR="build/reorganization"
PLAN="$PLAN_DIR/phase1-base1-fyr-move-plan.txt"
REPORT="$PLAN_DIR/phase1-base1-fyr-report.env"
APPLY=0

case "$MODE" in
  --dry-run|dry-run) APPLY=0 ;;
  --apply|apply) APPLY=1 ;;
  *) printf 'usage: sh scripts/reorganize-phase1-base1-fyr.sh --dry-run|--apply\n' >&2; exit 1 ;;
esac

fail() { printf 'reorganize-phase1-base1-fyr: %s\n' "$1" >&2; exit 1; }
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
  if [ "$APPLY" -eq 1 ]; then
    git mv "$src" "$dst"
  else
    printf '[dry-run] git mv %s %s\n' "$src" "$dst"
  fi
}

move_conflicting_root_file() {
  name="$1"
  dst="$2"
  if [ -e "$name" ] && [ ! -d "$name" ]; then
    move_if_exists "$name" "$dst"
  fi
}

write_doc() {
  path="$1"
  kind="$2"
  mkdir_logged "$(dirname "$path")"
  log_plan "write $path ($kind)"
  if [ "$APPLY" -eq 0 ]; then
    printf '[dry-run] write %s\n' "$path"
    return 0
  fi
  [ -f "$path" ] && return 0
  case "$kind" in
    junk-readme)
      cat > "$path" <<'EOF'
# Junk / preservation area

This directory preserves aged, duplicate, unclear, generated, or historical files that should not drive the active Phase1/Base1/Fyr workflow.

Files here are not deleted. They can be revived later with a clear reason.
EOF
      ;;
    phase1-readme)
      cat > "$path" <<'EOF'
# Phase1

Phase1 is the operator/runtime system: shell, UI, renderer, local tools, and user-facing environment.

Active Rust implementation currently remains in the root `src/` tree until a separate Rust module migration plan is approved.

The old root `phase1` launcher was moved to `scripts/launchers/phase1` so this directory can become the logical Phase1 project home.
EOF
      ;;
    base1-readme)
      cat > "$path" <<'EOF'
# Base1

Base1 is the boot, hardware, recovery, and runtime foundation for Phase1.

X200, QEMU, USB media, framebuffer boot, recovery, and hardware evidence belong here.
EOF
      ;;
    fyr-readme)
      cat > "$path" <<'EOF'
# Fyr

Fyr is the Phase1-native language track.

It is preserved during the cleanup and resumes after Base1 and Phase1 are coherent.
EOF
      ;;
    scripts-readme)
      cat > "$path" <<'EOF'
# Scripts

Use `scripts/phase1-base1.sh` as the active front door during the Phase1/Base1 focus period.

Historical scripts are being sorted into active, project-specific, or junk preservation paths.

Legacy launchers are preserved under `scripts/launchers/`.
EOF
      ;;
    active-os-path)
      cat > "$path" <<'EOF'
# Active Phase1/Base1 path

The active path is tracked in `TRACKER.md` and `docs/os/PHASE1_BASE1_UNIFICATION_PLAN.md`.

Historical B-series files remain preserved while the repository is reorganized.
EOF
      ;;
    *) fail "unknown write_doc kind: $kind" ;;
  esac
}

printf 'Phase1/Base1/Fyr reorganization\n'
printf 'mode: %s\n' "$MODE"
printf 'plan: %s\n\n' "$PLAN"

# First resolve root name conflicts. The repository has historically had a
# root file named `phase1`; a directory with the same name cannot be created
# until that launcher is preserved elsewhere.
move_conflicting_root_file "phase1" "scripts/launchers/phase1"
move_conflicting_root_file "fyr" "scripts/launchers/fyr"
move_conflicting_root_file "shared" "junk/root-archive/shared"
move_conflicting_root_file "junk" "junk/root-archive/junk-file"

# New primary directories.
for dir in \
  phase1/docs phase1/tools \
  base1/docs base1/scripts base1/evidence base1/targets \
  fyr/docs fyr/examples fyr/tests \
  shared/assets shared/docs shared/tooling \
  junk/legacy/docs junk/legacy/scripts junk/experiments junk/old-docs junk/old-scripts junk/generated-backups junk/root-archive \
  scripts/active scripts/base1 scripts/phase1 scripts/fyr scripts/maintenance scripts/launchers
 do
  mkdir_logged "$dir"
done

write_doc "junk/README.md" junk-readme
write_doc "phase1/README.md" phase1-readme
write_doc "base1/README.md" base1-readme
write_doc "fyr/README.md" fyr-readme
write_doc "scripts/README.md" scripts-readme

# Root generated backups / obvious local patch backups.
for f in ./*.bak ./*.old ./*.old1 ./*.old2; do
  [ -e "$f" ] || continue
  base="$(basename "$f")"
  move_if_exists "$f" "junk/generated-backups/$base"
done
for f in scripts/*.bak src/*.bak docs/*.bak tests/*.bak; do
  [ -e "$f" ] || continue
  base="$(printf '%s' "$f" | tr '/' '__')"
  move_if_exists "$f" "junk/generated-backups/$base"
done

# Root docs: preserve active control docs; move non-control root docs into logical homes.
move_if_exists "PHASE1_NATIVE_LANGUAGE.md" "fyr/docs/PHASE1_NATIVE_LANGUAGE.md"
move_if_exists "LEARNING.md" "phase1/docs/LEARNING.md"
move_if_exists "EDGE.md" "junk/legacy/docs/EDGE.md"
move_if_exists "FEATURE_STATUS.md" "phase1/docs/FEATURE_STATUS.md"
move_if_exists "QUALITY.md" "shared/docs/QUALITY.md"
move_if_exists "SECURITY_REVIEW.md" "shared/docs/SECURITY_REVIEW.md"
move_if_exists "SECURITY.md" "shared/docs/SECURITY.md"
move_if_exists "CONTRIBUTING.md" "shared/docs/CONTRIBUTING.md"

# Keep LICENSE, Cargo.toml, Cargo.lock, README.md, TRACKER.md, FOCUS.md, .github, src, tests in place for first pass.

# Script sorting: move older B-series script experiments to junk, keep active/current helpers in scripts/ for now.
for f in scripts/x200-b0*.sh scripts/x200-b1*.sh scripts/x200-b2*.sh scripts/base1-b1*.sh scripts/base1-b2*.sh; do
  [ -e "$f" ] || continue
  base="$(basename "$f")"
  case "$base" in
    x200-b47-*|x200-b48-*|stage-x200-kernel*|phase1-base1.sh|black-phase1*) continue ;;
  esac
  move_if_exists "$f" "junk/legacy/scripts/$base"
done

# Docs OS historical B-series are preserved in place for now; create active index marker.
write_doc "docs/os/ACTIVE_PHASE1_BASE1_PATH.md" active-os-path

cat > "$REPORT" <<EOF
PHASE1_REORG_MODE=$MODE
PHASE1_REORG_APPLY=$APPLY
PHASE1_REORG_PLAN=$PLAN
PHASE1_REORG_RESULT=$([ "$APPLY" -eq 1 ] && printf applied || printf dry_run)
PHASE1_REORG_NEXT=cargo_check_and_path_fix
EOF

printf '\nReorganization %s complete.\n' "$([ "$APPLY" -eq 1 ] && printf applied || printf dry-run)"
printf 'Plan: %s\n' "$PLAN"
printf 'Report: %s\n' "$REPORT"
if [ "$APPLY" -eq 0 ]; then
  printf '\nReview the plan, then run:\n'
  printf '  sh scripts/reorganize-phase1-base1-fyr.sh --apply\n'
else
  printf '\nNext checks:\n'
  printf '  git status --short\n'
  printf '  cargo check --all-targets\n'
  printf '  sh scripts/phase1-base1.sh status\n'
fi
