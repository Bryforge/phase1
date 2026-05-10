# Base1 Recovery and OS Foundation Manual

> **Status:** Roadmap and design index.
>
> **Validation:** Links to current Base1 design docs, dry-run scripts, inventory, test inventory, migration table, readiness checklist, compatibility maps, validation runbook, validation report template, validation reports archive, preview stack runbook, preview checks, and future validation reports.
>
> **Non-claims:** Base1 is not currently documented here as a released bootable daily-driver image, finished secure OS replacement, or destructive installer-ready system.

Base1 is the planned minimal host foundation for future Phase1-first bootable environments. The first practical documentation goal is to keep Base1 precise: read-only base, writable Phase1 state, recovery shell, rollback planning, image provenance, target identity verification, and explicit operator confirmation.

## Planned chapters

1. Scope and non-claims.
2. Base1 principles.
3. Layer model.
4. Boot flow.
5. Recovery shell.
6. Recovery USB planning.
7. Image provenance.
8. Rollback metadata.
9. Installer policy.
10. Hardware targets.
11. Validation reports.
12. Roadmap gates.

## Source-of-truth links

- [`DOCUMENTATION_MAP.md`](DOCUMENTATION_MAP.md)
- [`INVENTORY.md`](INVENTORY.md)
- [`TEST_INVENTORY.md`](TEST_INVENTORY.md)
- [`MIGRATION_TABLE.md`](MIGRATION_TABLE.md)
- [`REORGANIZATION_READINESS.md`](REORGANIZATION_READINESS.md)
- [`DOCUMENTATION_ORGANIZATION_PLAN.md`](DOCUMENTATION_ORGANIZATION_PLAN.md)
- [`ROOT_COMPATIBILITY_MAP.md`](ROOT_COMPATIBILITY_MAP.md)
- [`releases/README.md`](releases/README.md)
- [`READINESS_MATRIX.md`](READINESS_MATRIX.md)
- [`VALIDATION_RUNBOOK.md`](VALIDATION_RUNBOOK.md)
- [`VALIDATION_REPORT_TEMPLATE.md`](VALIDATION_REPORT_TEMPLATE.md)
- [`PREVIEW_STACK_RUNBOOK.md`](PREVIEW_STACK_RUNBOOK.md)
- [`PREVIEW_CHECKS.md`](PREVIEW_CHECKS.md)
- [`validation/README.md`](validation/README.md)
- [`../MANUAL_ROADMAP.md`](../MANUAL_ROADMAP.md)
- [`../security/TRUST_MODEL.md`](../security/TRUST_MODEL.md)
- [`../../base1/README.md`](../../base1/README.md)
- [`../../base1/SECURITY_MODEL.md`](../../base1/SECURITY_MODEL.md)
- [`../../base1/HARDWARE_TARGETS.md`](../../base1/HARDWARE_TARGETS.md)
- [`../../base1/ROADMAP.md`](../../base1/ROADMAP.md)
- [`../os/ROADMAP.md`](../os/ROADMAP.md)

## Compatibility rule

Root-level Base1 checkpoint notes remain compatibility paths. Use [`ROOT_COMPATIBILITY_MAP.md`](ROOT_COMPATIBILITY_MAP.md) to see which root files mirror organized notes under [`releases/`](releases/).

## Inventory rule

Use [`INVENTORY.md`](INVENTORY.md) before broader organization work. It groups known Base1 docs, scripts, tests, release/checkpoint notes, and compatibility paths so organization can proceed without losing recoverability.

Use [`TEST_INVENTORY.md`](TEST_INVENTORY.md) before moving or renaming tests. It groups known Base1-related Rust tests by behavior area so coverage remains visible during organization work.

Use [`MIGRATION_TABLE.md`](MIGRATION_TABLE.md) before relocating any Base1 group. It records current paths, proposed organized paths, compatibility decisions, and readiness blockers.

## Base1 wording rule

Use `planned`, `design`, `dry-run`, `preview`, or `validated` according to evidence. Do not call Base1 bootable, daily-driver ready, recovery-complete, or installer-ready without release artifacts and validation reports.

## Promotion rule

Use [`READINESS_MATRIX.md`](READINESS_MATRIX.md) before strengthening Base1 wording. A page may only move from roadmap to design, dry-run, preview, or validated when the linked evidence supports that level.

## Runbook rule

Use [`VALIDATION_RUNBOOK.md`](VALIDATION_RUNBOOK.md) for documentation-only Base1 checks. The runbook verifies docs structure and guardrails only; it does not validate boot, hardware, recovery, rollback, image, installer, persistence, or daily-driver behavior.

Use [`PREVIEW_STACK_RUNBOOK.md`](PREVIEW_STACK_RUNBOOK.md) for the current safe emulator-preview stack. The preview stack runbook covers input checks, bundle generation, doctor checks, dry-run gating, provenance, and checksum verification; it does not validate boot, hardware, recovery, installer behavior, or daily-driver readiness.

Use [`PREVIEW_CHECKS.md`](PREVIEW_CHECKS.md) after syncing `edge/stable` to run the current preview-stack test set and safe manual smoke checklist. The checks page does not validate boot, hardware, recovery, installer behavior, or daily-driver readiness.

## Report rule

Use [`VALIDATION_REPORT_TEMPLATE.md`](VALIDATION_REPORT_TEMPLATE.md) when recording Base1 evidence so reports name the scope, target, commands, result, observations, evidence links, boundaries, and promotion recommendation.

Store future Base1 reports under [`validation/`](validation/) so evidence remains discoverable and reviewable.
- [Base1 real-device read-only validation plan](real-device/READONLY_VALIDATION_PLAN.md)
- [Real-device read-only report template](real-device/READONLY_REPORT_TEMPLATE.md)
- [Real-device read-only validation bundle report](real-device/READONLY_VALIDATION_BUNDLE_REPORT.md)
- [Real-device read-only validation index](real-device/README.md)
- [Documentation map](DOCUMENTATION_MAP.md)
- [Base1 inventory](INVENTORY.md)
- [Base1 test inventory](TEST_INVENTORY.md)
- [Base1 migration table](MIGRATION_TABLE.md)
- [Reorganization readiness checklist](REORGANIZATION_READINESS.md)
- [Documentation organization plan](DOCUMENTATION_ORGANIZATION_PLAN.md)
- [Root compatibility map](ROOT_COMPATIBILITY_MAP.md)
