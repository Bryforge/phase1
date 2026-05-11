#!/usr/bin/env sh
# Base1 documentation integrity gate.
#
# This check is read-only. It verifies the current Base1 documentation layout,
# inventory, test inventory, test inventory reporter, migration table, script
# compatibility plan, link check strategy, readiness checklist, core references,
# release-note mirrors, root compatibility paths, and dry-run guardrails before
# file reorganization continues.

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
    docs/base1/ROOT_COMPATIBILITY_MAP.md \
    docs/base1/INVENTORY.md \
    docs/base1/TEST_INVENTORY.md \
    docs/base1/MIGRATION_TABLE.md \
    docs/base1/SCRIPT_COMPATIBILITY_PLAN.md \
    docs/base1/LINK_CHECK_STRATEGY.md \
    docs/base1/REORGANIZATION_READINESS.md \
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

check_root_compatibility_docs() {
  for file in \
    RELEASE_BASE1_LIBREBOOT_READONLY_V1.md \
    RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md \
    RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md \
    RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md \
    RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md \
    RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md
  do
    check_file "$file"
  done
}

check_release_docs() {
  for file in \
    docs/base1/releases/README.md \
    docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md \
    docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md \
    docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md \
    docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md \
    docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md \
    docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md
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
    scripts/base1-doc-integrity.sh \
    scripts/base1-link-check.sh \
    scripts/base1-test-inventory.sh \
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
  check_contains docs/base1/README.md 'ROOT_COMPATIBILITY_MAP.md'
  check_contains docs/base1/README.md 'INVENTORY.md'
  check_contains docs/base1/README.md 'TEST_INVENTORY.md'
  check_contains docs/base1/README.md 'MIGRATION_TABLE.md'
  check_contains docs/base1/README.md 'SCRIPT_COMPATIBILITY_PLAN.md'
  check_contains docs/base1/README.md 'LINK_CHECK_STRATEGY.md'
  check_contains docs/base1/README.md 'REORGANIZATION_READINESS.md'
  check_contains docs/base1/README.md 'releases/README.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md '../../base1/README.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md '../../base1/NETWORK_LOCKDOWN_DRY_RUN.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'INVENTORY.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'TEST_INVENTORY.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'MIGRATION_TABLE.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'SCRIPT_COMPATIBILITY_PLAN.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'LINK_CHECK_STRATEGY.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'REORGANIZATION_READINESS.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'ROOT_COMPATIBILITY_MAP.md'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'docs/base1/releases/'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'sh scripts/base1-doc-integrity.sh'
  check_contains docs/base1/INVENTORY.md 'Release and checkpoint notes'
  check_contains docs/base1/INVENTORY.md 'Base1 scripts'
  check_contains docs/base1/INVENTORY.md 'Test groups'
  check_contains docs/base1/TEST_INVENTORY.md 'Base1 test inventory'
  check_contains docs/base1/TEST_INVENTORY.md 'sh scripts/base1-test-inventory.sh'
  check_contains docs/base1/TEST_INVENTORY.md 'quality_base1_docs_gate.rs'
  check_contains docs/base1/TEST_INVENTORY.md 'base1_root_compatibility_map_docs.rs'
  check_contains docs/base1/TEST_INVENTORY.md 'Recovery USB emergency-shell tests'
  check_contains docs/base1/MIGRATION_TABLE.md 'Base1 migration table'
  check_contains docs/base1/MIGRATION_TABLE.md 'Compatibility decision'
  check_contains docs/base1/MIGRATION_TABLE.md 'keep root compatibility path'
  check_contains docs/base1/MIGRATION_TABLE.md 'scripts/base1-doc-integrity.sh'
  check_contains docs/base1/MIGRATION_TABLE.md 'No move until links, tests, and compatibility shims are planned.'
  check_contains docs/base1/SCRIPT_COMPATIBILITY_PLAN.md 'Base1 script compatibility plan'
  check_contains docs/base1/SCRIPT_COMPATIBILITY_PLAN.md 'Do not move Base1 scripts until compatibility wrappers are planned and tested.'
  check_contains docs/base1/SCRIPT_COMPATIBILITY_PLAN.md 'scripts/base1-*.sh'
  check_contains docs/base1/SCRIPT_COMPATIBILITY_PLAN.md 'Preserve command-line arguments exactly.'
  check_contains docs/base1/SCRIPT_COMPATIBILITY_PLAN.md 'Wrapper template'
  check_contains docs/base1/LINK_CHECK_STRATEGY.md 'Base1 link-check strategy'
  check_contains docs/base1/LINK_CHECK_STRATEGY.md 'Do not broadly reorganize Base1 markdown until link checking is available'
  check_contains docs/base1/LINK_CHECK_STRATEGY.md 'scripts/base1-link-check.sh'
  check_contains docs/base1/LINK_CHECK_STRATEGY.md 'Fail on missing local targets.'
  check_contains docs/base1/LINK_CHECK_STRATEGY.md 'Stay read-only.'
  check_contains scripts/base1-link-check.sh 'mode: read-only'
  check_contains scripts/base1-link-check.sh 'missing local link target'
  check_contains scripts/base1-link-check.sh 'external-links: skipped'
  check_contains scripts/base1-test-inventory.sh 'mode: read-only'
  check_contains scripts/base1-test-inventory.sh 'tests/base1_*.rs'
  check_contains scripts/base1-test-inventory.sh 'tests/quality_base1_*.rs'
  check_contains scripts/base1-test-inventory.sh 'inventory complete; no files were changed'
  check_contains docs/base1/REORGANIZATION_READINESS.md 'Base1 is not ready for a full reorganization yet.'
  check_contains docs/base1/REORGANIZATION_READINESS.md 'complete inventory'
  check_contains docs/base1/ROOT_COMPATIBILITY_MAP.md 'RELEASE_BASE1_LIBREBOOT_READONLY_V1.md'
  check_contains docs/base1/ROOT_COMPATIBILITY_MAP.md 'docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md'
  check_contains docs/base1/ROOT_COMPATIBILITY_MAP.md 'RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md'
  check_contains docs/base1/ROOT_COMPATIBILITY_MAP.md 'docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md'
  check_contains base1/README.md 'NETWORK_LOCKDOWN_DRY_RUN.md'
  check_contains base1/README.md 'scripts/base1-network-lockdown-dry-run.sh'
  check_contains docs/os/BASE1_DRY_RUN_COMMANDS.md 'scripts/base1-network-lockdown-dry-run.sh --dry-run'
  check_contains base1/NETWORK_LOCKDOWN_DRY_RUN.md 'Read-only guarantee'
  check_contains base1/NETWORK_LOCKDOWN_DRY_RUN.md 'remote access can be lost'
  check_contains base1/RECOVERY_USB_COMMAND_INDEX.md 'docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md'
  check_contains base1/LIBREBOOT_MILESTONE.md 'docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md'
  check_contains scripts/base1-network-lockdown-dry-run.sh 'refusing to run without --dry-run'
  check_contains scripts/base1-network-lockdown-dry-run.sh "info 'writes: no'"
}

check_non_claims() {
  check_contains docs/base1/DOCUMENTATION_MAP.md 'Not installer-ready'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'Not hardware-validated'
  check_contains docs/base1/DOCUMENTATION_MAP.md 'No destructive disk writes'
  check_contains docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md 'Keep existing Base1 markdown files available'
  check_contains docs/base1/INVENTORY.md 'does not make Base1 installer-ready'
  check_contains docs/base1/TEST_INVENTORY.md 'does not make Base1 installer-ready'
  check_contains docs/base1/MIGRATION_TABLE.md 'does not make Base1 installer-ready'
  check_contains docs/base1/SCRIPT_COMPATIBILITY_PLAN.md 'does not make Base1 installer-ready'
  check_contains docs/base1/LINK_CHECK_STRATEGY.md 'does not make Base1 installer-ready'
  check_contains docs/base1/REORGANIZATION_READINESS.md 'does not make Base1 installer-ready'
  check_contains docs/base1/releases/README.md 'No destructive disk writes'
  check_contains docs/base1/ROOT_COMPATIBILITY_MAP.md 'Base1 installer-ready'
  check_contains base1/README.md 'They do not yet constitute a destructive installer or complete operating system image.'
}

check_core_docs
check_root_compatibility_docs
check_release_docs
check_real_device_docs
check_scripts
check_references
check_non_claims

info 'integrity complete; Base1 docs, inventory, test inventory, test inventory reporter, migration table, script compatibility plan, link-check strategy, link checker, readiness checklist, root compatibility paths, release mirrors, and dry-run references are present'
info 'writes: no'
