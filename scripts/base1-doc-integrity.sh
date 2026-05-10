#!/usr/bin/env sh
# Base1 documentation integrity gate.
#
# This check is read-only. It verifies the current Base1 documentation layout,
# core references, and dry-run guardrails before file reorganization continues.

set -eu

info() {
  printf 'base1-doc-integrity: %s\n' "$1"
}

fail() {
  printf 'base1-doc-integrity error: %s\n' "$1" >&2
  exit 1
}

check_file() {
  [ -f "$1" ] || fail "missing required file: $1"
  [ -s "$1" ] || fail "required file is empty: $1"
  info "file ok: $1"
}

check_contains() {
  file=$1
  needle=$2
  grep -F "$needle" "$file" >/dev/null 2>&1 || fail "$file does not contain required text: $needle"
  info "reference ok: $file -> $needle"
}

check_script_syntax() {
  file=$1
  check_file "$file"
  sh -n "$file"
  info "script syntax ok: $file"
}

check_core_docs() {
  for file in \
    base1/README.md \
    base1/SECURITY_MODEL.md \
    base1/HARDWARE_TARGETS.md \
    base1/PHASE1_COMPATIBILITY.md \
    base1/ROADMAP.md \
    base1/NETWORK_LOCKDOWN_DRY_RUN.md \
    docs/base1/README.md \
    docs/base1/DOCUMENTATION_MAP.md \
    docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md \
    docs/base1/VALIDATION_RUNBOOK.md \
    docs/base1/VALIDATION_REPORT_TEMPLATE.md \
    docs/base1/VALIDATION_REPORTS.md \
    docs/os/BASE1_DRY_RUN_COMMANDS.md \
    docs/os/BASE1_IMAGE_BUILDER.md \
    docs/os/BASE1_INSTALLER_DRY_RUN.md \
    docs/os/BASE1_RECOVERY_COMMAND.md \
    docs/os/BASE1_STORAGE_LAYOUT_CHECKER.md \
    docs/os/BASE1_ROLLBACK_METADATA.md
  do
    check_file "$file"
  done
}

check_real_device_docs() {
  for file in \
    docs/base1/real-device/README.md \
    docs/base1/real-device/READONLY_VALIDATION_PLAN.md \
    docs/base1/real-device/READONLY_REPORT_TEMPLATE.md \
    docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md \
    docs/base1/real-device/RUNBOOK.md \
    docs/base1/real-device/CHECKLIST.md \
    docs/base1/real-device/STATUS_SUMMARY.md
  do
    check_file "$file"
  done
}

check_scripts() {
  for file in \
    scripts/base1-preflight.sh \
    scripts/base1-install-dry-run.sh \
    scripts/base1-recovery-dry-run.sh \
    scripts/base1-storage-layout-dry-run.sh \
    scripts/base1-rollback-metadata-dry-run.sh \
    scripts/base1-network-lockdown-dry-run.sh \
    scripts/base1-preview-stack.sh \
    scripts/base1-preview-gate.sh \
    scripts/base1-preview-verify.sh \
    scripts/base1-real-device-readonly-preview.sh \
    scripts/base1-real-device-readonly-report.sh \
    scripts/base1-real-device-readonly-validation-bundle.sh \
    scripts/base1-real-device-readonly-doctor.sh
  do
    check_script_syntax "$file"
  done
}

check_references() {
  check_contains docs/base1/README.md 'DOCUMENTATION_MAP.md'
  check_contains docs/base1/README.md 'DOCUMENTATION_ORGANIZATION_PLAN.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md '../../base1/README.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md '../../base1/NETWORK_LOCKDOWN_DRY_RUN.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'sh scripts/base1-doc-integrity.sh'
  check_contains base1/README.md 'NETWORK_LOCKDOWN_DRY_RUN.md'
  check_contains base1/README.md 'scripts/base1-network-lockdown-dry-run.sh'
  check_contains docs/os/BASE1_DRY_RUN_COMMANDS.md 'scripts/base1-network-lockdown-dry-run.sh --dry-run'
  check_contains base1/NETWORK_LOCKDOWN_DRY_RUN.md 'Read-only guarantee'
  check_contains base1/NETWORK_LOCKDOWN_DRY_RUN.md 'remote access can be lost'
  check_contains scripts/base1-network-lockdown-dry-run.sh 'refusing to run without --dry-run'
  check_contains scripts/base1-network-lockdown-dry-run.sh "info 'writes: no'"
}

check_non_claims() {
  check_contains docs/base1/DOCUMENTATION_MAP.md 'Not installer-ready'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'Not hardware-validated'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'No destructive disk writes'
  check_contains docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md 'Do not move existing Base1 markdown files unless the same PR updates every link, test, and index reference.'
  check_contains base1/README.md 'They do not yet constitute a destructive installer or complete operating system image.'
}

check_core_docs
check_real_device_docs
check_scripts
check_references
check_non_claims

info 'integrity complete; Base1 docs and dry-run references are present'
info 'writes: no'
