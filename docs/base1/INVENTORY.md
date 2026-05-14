# Base1 inventory

Status: active inventory seed
Scope: Base1 docs, scripts, tests, release/checkpoint notes, and compatibility paths

## Purpose

This inventory is the next safety layer before broader Base1 reorganization.

It groups known Base1 material without removing anything. It is not yet a complete repository tree dump; it is a preservation-first inventory seed that can be expanded before any full reorganization.

## Inventory rules

- Do not remove existing files during inventory work.
- Keep release archive paths available.
- Prefer adding mirrors, indexes, and compatibility maps before moving anything.
- Update this inventory when Base1 docs, scripts, tests, or release/checkpoint notes are added.
- Run `sh scripts/quality-check.sh base1-docs` after inventory or organization changes.

## Core Base1 docs

| Path | Group | Notes |
| --- | --- | --- |
| `base1/README.md` | Core | Base1 overview and current foundation status. |
| `base1/SECURITY_MODEL.md` | Core | Security model and trust boundary. |
| `base1/HARDWARE_TARGETS.md` | Core | Raspberry Pi and ThinkPad X200-class target notes. |
| `base1/PHASE1_COMPATIBILITY.md` | Core | Base1/Phase1 compatibility contract. |
| `base1/ROADMAP.md` | Core | Base1 staged roadmap. |
| `base1/NETWORK_LOCKDOWN_DRY_RUN.md` | Core / network | Read-only network lockdown preview contract. |
| `base1/config/base1-secure-profile.toml` | Config | Secure profile draft. |
| `base1/systemd/phase1-base1.service` | Service | Hardened service template. |

## Base1 organization and validation docs

| Path | Group | Notes |
| --- | --- | --- |
| `docs/base1/README.md` | Manual | Base1 recovery and OS foundation manual index. |
| `docs/base1/DOCUMENTATION_MAP.md` | Organization | Current canonical navigation map. |
| `docs/base1/TEST_INVENTORY.md` | Organization | Base1 test inventory and grouped test coverage map. |
| `docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md` | Organization | Preservation-first organization plan. |
| `docs/base1/RELEASE_ARCHIVE_MAP.md` | Organization | Root path to organized mirror map. |
| `docs/base1/REORGANIZATION_READINESS.md` | Organization | Full-reorganization readiness criteria. |
| `docs/base1/READINESS_MATRIX.md` | Validation | Evidence/maturity matrix. |
| `docs/base1/VALIDATION_RUNBOOK.md` | Validation | Documentation-only validation runbook. |
| `docs/base1/VALIDATION_REPORT_TEMPLATE.md` | Validation | Base1 validation report template. |
| `docs/base1/VALIDATION_REPORTS.md` | Validation | Validation reports index. |
| `docs/base1/PREVIEW_STACK_RUNBOOK.md` | Preview | Emulator-preview stack runbook. |
| `docs/base1/PREVIEW_CHECKS.md` | Preview | Preview stack checks. |
| `docs/base1/validation/README.md` | Validation | Validation report directory index. |

## OS-track Base1 design slices

| Path | Group | Notes |
| --- | --- | --- |
| `docs/os/BASE1_IMAGE_BUILDER.md` | OS track | Image-builder design. |
| `docs/os/BASE1_INSTALLER_DRY_RUN.md` | OS track | Installer dry-run design. |
| `docs/os/BASE1_RECOVERY_COMMAND.md` | OS track | Recovery command design. |
| `docs/os/BASE1_STORAGE_LAYOUT_CHECKER.md` | OS track | Storage layout checker design. |
| `docs/os/BASE1_ROLLBACK_METADATA.md` | OS track | Rollback metadata design. |
| `docs/os/BASE1_DRY_RUN_COMMANDS.md` | OS track | Base1 dry-run command index. |
| `docs/os/ROADMAP.md` | OS track | Phase1 operating-system track roadmap. |

## Libreboot docs

| Path | Group | Notes |
| --- | --- | --- |
| `base1/LIBREBOOT_PROFILE.md` | Libreboot | X200-class Libreboot profile. |
| `base1/LIBREBOOT_PREFLIGHT.md` | Libreboot | Read-only preflight notes. |
| `base1/LIBREBOOT_GRUB_RECOVERY.md` | Libreboot | GRUB recovery notes. |
| `base1/LIBREBOOT_OPERATOR_CHECKLIST.md` | Libreboot | Operator checklist. |
| `base1/LIBREBOOT_MILESTONE.md` | Libreboot | Read-only milestone checkpoint. |
| `base1/LIBREBOOT_DOCS_SUMMARY.md` | Libreboot | Libreboot docs summary. |
| `base1/LIBREBOOT_QUICKSTART.md` | Libreboot | Quickstart. |
| `base1/LIBREBOOT_COMMAND_INDEX.md` | Libreboot | Command index. |
| `base1/LIBREBOOT_VALIDATION_REPORT.md` | Libreboot | Validation report template. |

## Recovery USB docs

| Path | Group | Notes |
| --- | --- | --- |
| `base1/RECOVERY_USB_DESIGN.md` | Recovery USB | Main recovery USB design. |
| `base1/RECOVERY_USB_COMMAND_INDEX.md` | Recovery USB | Recovery USB command index. |
| `base1/RECOVERY_USB_VALIDATION_REPORT.md` | Recovery USB | Validation report. |
| `base1/RECOVERY_USB_HARDWARE_CHECKLIST.md` | Recovery USB / hardware | Hardware validation checklist. |
| `base1/RECOVERY_USB_HARDWARE_SUMMARY.md` | Recovery USB / hardware | Hardware validation summary. |
| `base1/RECOVERY_USB_TARGET_SELECTION.md` | Recovery USB / target | Target-device selection design. |
| `base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md` | Recovery USB / target | Target command index. |
| `base1/RECOVERY_USB_TARGET_SUMMARY.md` | Recovery USB / target | Target selection summary. |
| `base1/RECOVERY_USB_IMAGE_PROVENANCE.md` | Recovery USB / image | Image provenance and checksum design. |
| `base1/RECOVERY_USB_IMAGE_SUMMARY.md` | Recovery USB / image | Image provenance summary. |
| `base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md` | Recovery USB / image | Image command index. |
| `base1/RECOVERY_USB_EMERGENCY_SHELL.md` | Recovery USB / shell | Emergency shell behavior design. |
| `base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md` | Recovery USB / shell | Emergency shell summary. |
| `base1/RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md` | Recovery USB / shell | Emergency shell command index. |

## Real-device read-only docs

| Path | Group | Notes |
| --- | --- | --- |
| `docs/base1/real-device/README.md` | Real device | Read-only real-device index. |
| `docs/base1/real-device/READONLY_VALIDATION_PLAN.md` | Real device | Read-only validation plan. |
| `docs/base1/real-device/READONLY_REPORT_TEMPLATE.md` | Real device | Report template. |
| `docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md` | Real device | Validation bundle report. |
| `docs/base1/real-device/RUNBOOK.md` | Real device | Read-only runbook. |
| `docs/base1/real-device/CHECKLIST.md` | Real device | Read-only checklist. |
| `docs/base1/real-device/STATUS_SUMMARY.md` | Real device | Status summary. |
| `docs/base1/real-device/reports/2026-05-10-readonly-evidence-capture.md` | Real device | Evidence capture report. |

## Release and checkpoint notes

| Root compatibility path | Organized mirror |
| --- | --- |
| `RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` |
| `RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` |
| `RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md` |
| `N/A` | `docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md` |

Additional checkpoint material:

| Path | Group | Notes |
| --- | --- | --- |
| `docs/archive/checkpoints/DEVELOPMENT_CHECKPOINT_BASE1_RECOVERY_USB_READONLY_V1.md` | Checkpoint | Development checkpoint note retained at root. |
| `docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md` | Release/checkpoint | B6 X200 marker checkpoint release note with `phase1_marker_seen` and `not_claimed` boundary. |

## Base1 scripts

| Path | Group | Notes |
| --- | --- | --- |
| `scripts/base1-preflight.sh` | Base1 | Non-destructive host readiness checker. |
| `scripts/base1-phase1-run.sh` | Base1 | Hardened Phase1 launcher wrapper. |
| `scripts/base1-doc-integrity.sh` | Base1 / quality | Documentation integrity gate. |
| `scripts/base1-install-dry-run.sh` | Dry-run | Installer dry-run. |
| `scripts/base1-recovery-dry-run.sh` | Dry-run | Recovery dry-run. |
| `scripts/base1-storage-layout-dry-run.sh` | Dry-run | Storage layout dry-run. |
| `scripts/base1-rollback-metadata-dry-run.sh` | Dry-run | Rollback metadata dry-run. |
| `scripts/base1-network-lockdown-dry-run.sh` | Dry-run / network | Network lockdown dry-run. |
| `scripts/base1-preview-stack.sh` | Preview | Preview stack command. |
| `scripts/base1-preview-gate.sh` | Preview | Preview gate. |
| `scripts/base1-preview-verify.sh` | Preview | Preview verification. |
| `scripts/base1-boot-preview.sh` | Preview | Boot preview. |
| `scripts/base1-emulator-preview.sh` | Preview | Emulator preview. |
| `scripts/base1-emulator-doctor.sh` | Preview | Emulator doctor. |
| `scripts/base1-libreboot-docs.sh` | Libreboot | Docs command. |
| `scripts/base1-libreboot-index.sh` | Libreboot | Index command. |
| `scripts/base1-libreboot-checklist.sh` | Libreboot | Checklist command. |
| `scripts/base1-libreboot-preflight.sh` | Libreboot | Preflight command. |
| `scripts/base1-libreboot-validate.sh` | Libreboot | Validation bundle. |
| `scripts/base1-libreboot-report.sh` | Libreboot | Report command. |
| `scripts/base1-libreboot-milestone.sh` | Libreboot | Milestone command. |
| `scripts/base1-grub-recovery-dry-run.sh` | Libreboot / recovery | GRUB recovery dry-run. |
| `scripts/base1-recovery-usb-dry-run.sh` | Recovery USB | Recovery USB dry-run. |
| `scripts/base1-recovery-usb-index.sh` | Recovery USB | Index command. |
| `scripts/base1-recovery-usb-validate.sh` | Recovery USB | Validation bundle. |
| `scripts/base1-recovery-usb-hardware-summary.sh` | Recovery USB / hardware | Hardware summary. |
| `scripts/base1-recovery-usb-hardware-report.sh` | Recovery USB / hardware | Hardware report. |
| `scripts/base1-recovery-usb-hardware-validate.sh` | Recovery USB / hardware | Hardware validation. |
| `scripts/base1-recovery-usb-hardware-checklist.sh` | Recovery USB / hardware | Hardware checklist. |
| `scripts/base1-recovery-usb-target-dry-run.sh` | Recovery USB / target | Target dry-run. |
| `scripts/base1-recovery-usb-target-summary.sh` | Recovery USB / target | Target summary. |
| `scripts/base1-recovery-usb-target-validate.sh` | Recovery USB / target | Target validation. |
| `scripts/base1-recovery-usb-target-report.sh` | Recovery USB / target | Target report. |
| `scripts/base1-recovery-usb-image-summary.sh` | Recovery USB / image | Image summary. |
| `scripts/base1-recovery-usb-image-validate.sh` | Recovery USB / image | Image validation. |
| `scripts/base1-recovery-usb-image-report.sh` | Recovery USB / image | Image report. |
| `scripts/base1-recovery-usb-emergency-shell-summary.sh` | Recovery USB / shell | Emergency shell summary. |
| `scripts/base1-recovery-usb-emergency-shell-validate.sh` | Recovery USB / shell | Emergency shell validation. |
| `scripts/base1-recovery-usb-emergency-shell-report.sh` | Recovery USB / shell | Emergency shell report. |
| `scripts/base1-real-device-readonly-preview.sh` | Real device | Read-only preview. |
| `scripts/base1-real-device-readonly-report.sh` | Real device | Read-only report. |
| `scripts/base1-real-device-readonly-validation-bundle.sh` | Real device | Read-only validation bundle. |
| `scripts/base1-real-device-readonly-doctor.sh` | Real device | Read-only doctor. |

## Test groups

Known Base1-related tests should remain grouped by surfaced behavior:

- Base1 foundation and compatibility tests.
- Base1 docs and organization tests.
- Libreboot docs/script/release-note tests.
- Recovery USB hardware/target/image/emergency-shell docs and script tests.
- Real-device read-only tests.
- Quality gate tests for `base1-docs`.

## Next inventory work

This inventory should become complete before full reorganization. The next safe improvement is a generated or manually verified test inventory that lists every `tests/base1_*` and `tests/quality_base1_*` file.

## Non-claims

This inventory does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only improves repository organization safety.
