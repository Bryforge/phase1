#!/usr/bin/env sh
# Phase1 local system diagnostics.
#
# This script is local-only. It writes sanitized reports under build/diagnostics
# by default. It does not add, commit, push, upload, install packages, mutate
# devices, inspect arbitrary home directories, or collect credentials.

set -eu

MODE=quick
REPO_COPY=0
REPORT_DIR="build/diagnostics"
LATEST_REPORT="$REPORT_DIR/latest.md"
REPO_COPY_PATH="docs/diagnostics/LATEST_LOCAL_DIAGNOSTICS.md"
TAIL_LINES=${PHASE1_DIAGNOSTICS_TAIL_LINES:-120}

usage() {
  cat <<'EOF'
phase1 system diagnostics

Usage:
  sh scripts/phase1-system-diagnostics.sh [--quick|--full|--no-tests] [--repo-copy]

Modes:
  --quick       collect state and run focused fast validation checks
  --full        collect state and run cargo test --workspace --all-targets
  --no-tests    collect state only
  --repo-copy   copy the generated report to docs/diagnostics/LATEST_LOCAL_DIAGNOSTICS.md for manual review
  --help        show this help

Safety:
  local reports only; no upload, no git add/commit/push, no package install,
  no device writes, no firmware writes, no secret collection.
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --quick) MODE=quick ;;
    --full) MODE=full ;;
    --no-tests) MODE=no-tests ;;
    --repo-copy) REPO_COPY=1 ;;
    --help|-h) usage; exit 0 ;;
    *)
      printf 'unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

mkdir -p "$REPORT_DIR"
STAMP=$(date -u +%Y%m%dT%H%M%SZ 2>/dev/null || printf 'unknown-time')
REPORT="$REPORT_DIR/phase1-system-diagnostics-$STAMP.md"
TMP_OUTPUT="$REPORT_DIR/.last-command-output.tmp"

sanitize_stream() {
  if [ -n "${HOME:-}" ]; then
    sed "s|$HOME|~|g"
  else
    cat
  fi
}

append() {
  printf '%s\n' "$*" >> "$REPORT"
}

append_block() {
  title=$1
  shift
  append "### $title"
  append ""
  append '```text'
  "$@" 2>&1 | sanitize_stream >> "$REPORT" || true
  append '```'
  append ""
}

command_exists() {
  command -v "$1" >/dev/null 2>&1
}

run_check() {
  label=$1
  shift
  append "### $label"
  append ""
  append '```text'
  printf '$' >> "$REPORT"
  for part in "$@"; do
    printf ' %s' "$part" >> "$REPORT"
  done
  printf '\n' >> "$REPORT"

  if "$@" > "$TMP_OUTPUT" 2>&1; then
    status=0
  else
    status=$?
  fi

  printf 'exit-status: %s\n' "$status" >> "$REPORT"
  printf -- '--- output tail ---%s' "\n" >> "$REPORT"
  tail -n "$TAIL_LINES" "$TMP_OUTPUT" 2>/dev/null | sanitize_stream >> "$REPORT" || true
  append '```'
  append ""
  return 0
}

write_header() {
  append '# Phase1 system diagnostics report'
  append ""
  append "- generated_utc: $STAMP"
  append "- mode: $MODE"
  append "- report_path: $REPORT"
  append "- latest_path: $LATEST_REPORT"
  append "- writes: local-report-only"
  append "- upload: no"
  append "- git_mutation: no"
  append "- secrets_collection: no"
  append ""
}

write_boundaries() {
  append '## Safety boundaries and non-claims'
  append ""
  append '- This report is local-only unless the operator manually reviews and commits a repo-copy file.'
  append '- The script does not upload, add, commit, push, install packages, write devices, flash firmware, partition disks, or collect credentials.'
  append '- The report does not prove production readiness, installer readiness, daily-driver readiness, hardening, cryptographic completeness, hardware validation, recovery completion, release-candidate readiness, live self-updating, autonomous promotion, or safety for hostile code.'
  append ""
}

write_repo_state() {
  append '## Repository state'
  append ""
  if command_exists git && git rev-parse --show-toplevel >/dev/null 2>&1; then
    append_block 'Git identity' sh -c 'printf "branch: "; git branch --show-current 2>/dev/null || true; printf "commit: "; git rev-parse --short HEAD 2>/dev/null || true; printf "root: "; git rev-parse --show-toplevel 2>/dev/null || true'
    append_block 'Git status short' git status --short
    append_block 'Recent commits' sh -c 'git log --oneline -n 8 2>/dev/null || true'
  else
    append 'Git repository: unavailable'
    append ""
  fi
}

write_toolchain() {
  append '## Toolchain availability'
  append ""
  append '```text'
  for tool in sh git cargo rustc rustfmt python3; do
    if command_exists "$tool"; then
      printf '%-10s : present : ' "$tool" >> "$REPORT"
      case "$tool" in
        sh) printf 'posix shell\n' >> "$REPORT" ;;
        git) git --version 2>&1 | sanitize_stream >> "$REPORT" || true ;;
        cargo) cargo --version 2>&1 | sanitize_stream >> "$REPORT" || true ;;
        rustc) rustc --version 2>&1 | sanitize_stream >> "$REPORT" || true ;;
        rustfmt) rustfmt --version 2>&1 | sanitize_stream >> "$REPORT" || true ;;
        python3) python3 --version 2>&1 | sanitize_stream >> "$REPORT" || true ;;
      esac
    else
      printf '%-10s : missing\n' "$tool" >> "$REPORT"
    fi
  done
  append '```'
  append ""
}

write_file_presence() {
  append '## Core file presence'
  append ""
  append '```text'
  for path in \
    README.md \
    Cargo.toml \
    src/main.rs \
    src/boot_ui_static.rs \
    docs/diagnostics/SYSTEM_DIAGNOSTICS.md \
    docs/ui/PROMPT_GOTHIC_STARFIELD.md \
    docs/portal/FLOOR1_PORTALS.md \
    docs/base1/DOCUMENTATION_MAP.md \
    docs/base1/RELEASE_ARCHIVE_MAP.md \
    base1/README.md \
    base1/RECOVERY_USB_DESIGN.md \
    scripts/base1-doc-integrity.sh \
    scripts/phase1-system-diagnostics.sh \
    tests/prompt_gothic_starfield_contract.rs
  do
    if [ -s "$path" ]; then
      printf '%-64s ok\n' "$path" >> "$REPORT"
    else
      printf '%-64s missing-or-empty\n' "$path" >> "$REPORT"
    fi
  done
  append '```'
  append ""
}

run_quick_checks() {
  append '## Focused validation results'
  append ""
  if ! command_exists cargo; then
    append 'Cargo unavailable; focused validation skipped.'
    append ""
    return 0
  fi

  run_check 'Format check' cargo fmt --all -- --check
  run_check 'Prompt gothic/starfield contract' cargo test -p phase1 --test prompt_gothic_starfield_contract
  run_check 'Portal floor1 contract' cargo test -p phase1 --test portal_floor1_contract
  run_check 'Portal floor1 runtime' cargo test -p phase1 --test portal_floor1_runtime
  run_check 'Asset index docs' cargo test -p phase1 --test asset_index_docs
  run_check 'Base1 recovery USB design docs' cargo test -p phase1 --test base1_recovery_usb_design_docs
  run_check 'Base1 release pre-move checks docs' cargo test -p phase1 --test base1_release_pre_move_checks_docs
}

run_full_checks() {
  append '## Full validation results'
  append ""
  if ! command_exists cargo; then
    append 'Cargo unavailable; full validation skipped.'
    append ""
    return 0
  fi

  run_check 'Format check' cargo fmt --all -- --check
  run_check 'Workspace all targets' cargo test --workspace --all-targets
}

write_next_steps() {
  append '## Next operator commands'
  append ""
  append '```sh'
  append 'cat build/diagnostics/latest.md'
  append 'git status --short'
  append 'cargo test --workspace --all-targets'
  append '```'
  append ""
}

write_header
write_boundaries
write_repo_state
write_toolchain
write_file_presence

case "$MODE" in
  quick) run_quick_checks ;;
  full) run_full_checks ;;
  no-tests)
    append '## Validation results'
    append ""
    append 'Skipped by --no-tests.'
    append ""
    ;;
esac

write_next_steps
cp "$REPORT" "$LATEST_REPORT"
rm -f "$TMP_OUTPUT"

if [ "$REPO_COPY" -eq 1 ]; then
  mkdir -p docs/diagnostics
  cp "$REPORT" "$REPO_COPY_PATH"
  printf 'diagnostics repo-copy written: %s\n' "$REPO_COPY_PATH"
  printf 'review before committing; no git add/commit/push was run\n'
fi

printf 'diagnostics report written: %s\n' "$REPORT"
printf 'diagnostics latest: %s\n' "$LATEST_REPORT"
printf 'writes: local-report-only\n'
printf 'upload: no\n'
