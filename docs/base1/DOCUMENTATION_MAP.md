# Base1 Documentation Map

Status: active documentation index
Scope: Base1 docs, scripts, validation reports, read-only evidence paths, and integrity gates

## Purpose

This map organizes the Base1 documentation surface without moving files.

It is a navigation aid only. It does not change runtime behavior, validation status, installer readiness, or hardware claims.

## Canonical layout

Base1 currently uses three documentation zones:

| Zone | Purpose |
| --- | --- |
| `base1/` | Canonical Base1 security, hardware, compatibility, recovery, Libreboot, and operator design docs. |
| `docs/base1/` | Organization maps, validation runbooks, real-device read-only bundles, and evidence reports. |
| `docs/os/` | Phase1 operating-system-track docs and Base1 dry-run command design slices. |

Scripts remain under `scripts/` until a later tools reorganization is backed by tests and link updates.

## Core Base1 entry points

- [Base1 project README](../../base1/README.md)
- [Security model](../../base1/SECURITY_MODEL.md)
- [Hardware targets](../../base1/HARDWARE_TARGETS.md)
- [Phase1 compatibility contract](../../base1/PHASE1_COMPATIBILITY.md)
- [Network lockdown dry-run](../../base1/NETWORK_LOCKDOWN_DRY_RUN.md)
- [Base1 roadmap](../../base1/ROADMAP.md)

## Validation and organization

- [Base1 docs index](README.md)
- [Documentation organization plan](DOCUMENTATION_ORGANIZATION_PLAN.md)
- [Base1 validation runbook](VALIDATION_RUNBOOK.md)
- [Base1 validation report template](VALIDATION_REPORT_TEMPLATE.md)
- [Base1 validation reports index](VALIDATION_REPORTS.md)
- Integrity gate: `sh scripts/base1-doc-integrity.sh`

## OS-track dry-run design slices

- [Base1 image-builder design](../os/BASE1_IMAGE_BUILDER.md)
- [Installer dry-run](../os/BASE1_INSTALLER_DRY_RUN.md)
- [Recovery command](../os/BASE1_RECOVERY_COMMAND.md)
- [Storage layout checker](../os/BASE1_STORAGE_LAYOUT_CHECKER.md)
- [Rollback metadata](../os/BASE1_ROLLBACK_METADATA.md)
- [Dry-run command index](../os/BASE1_DRY_RUN_COMMANDS.md)

## Real-device read-only track

- [Real-device read-only index](real-device/README.md)
- [Read-only validation plan](real-device/READONLY_VALIDATION_PLAN.md)
- [Read-only report template](real-device/READONLY_REPORT_TEMPLATE.md)
- [Read-only validation bundle report](real-device/READONLY_VALIDATION_BUNDLE_REPORT.md)
- [Read-only runbook](real-device/RUNBOOK.md)
- [Read-only checklist](real-device/CHECKLIST.md)
- [Read-only status summary](real-device/STATUS_SUMMARY.md)
- [Read-only evidence capture report](real-device/reports/2026-05-10-readonly-evidence-capture.md)

## Real-device read-only scripts

- `scripts/base1-real-device-readonly-preview.sh`
- `scripts/base1-real-device-readonly-report.sh`
- `scripts/base1-real-device-readonly-validation-bundle.sh`
- `scripts/base1-real-device-readonly-doctor.sh`

## Base1 dry-run scripts

- `scripts/base1-preflight.sh`
- `scripts/base1-install-dry-run.sh`
- `scripts/base1-recovery-dry-run.sh`
- `scripts/base1-storage-layout-dry-run.sh`
- `scripts/base1-rollback-metadata-dry-run.sh`
- `scripts/base1-network-lockdown-dry-run.sh`
- `scripts/base1-recovery-usb-dry-run.sh`
- `scripts/base1-preview-stack.sh`
- `scripts/base1-preview-gate.sh`
- `scripts/base1-preview-verify.sh`

## Promotion rule

This map may only improve discoverability.

It does not promote Base1 to installer-ready, hardware-validated, or daily-driver status.

## Non-claims

- Not installer-ready
- Not hardware-validated
- Not daily-driver ready
- No destructive disk writes
- No real-device write path
