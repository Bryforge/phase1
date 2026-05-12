# Base1 post-reorganization layout

Status: proposed stable layout
Scope: Base1 documentation, scripts, tests, release/checkpoint notes, and compatibility paths

## Purpose

This document names the intended stable public paths after Base1 organization work matures.

It is a layout target, not a move instruction. Current paths remain recoverable unless explicit future approval says otherwise.

## Stable public paths

These paths should remain stable for users, contributors, and existing links:

| Path | Role |
| --- | --- |
| `base1/README.md` | Base1 public overview. |
| `docs/base1/README.md` | Base1 manual and source-of-truth index. |
| `docs/base1/DOCUMENTATION_MAP.md` | Base1 navigation map. |
| `docs/base1/INVENTORY.md` | Base1 inventory. |
| `docs/base1/TEST_INVENTORY.md` | Base1 test inventory. |
| `docs/base1/MIGRATION_TABLE.md` | Migration planning table. |
| `docs/base1/SCRIPT_COMPATIBILITY_PLAN.md` | Script compatibility plan. |
| `docs/base1/LINK_CHECK_STRATEGY.md` | Link-check strategy. |
| `docs/base1/REORGANIZATION_READINESS.md` | Reorganization readiness checklist. |
| `docs/base1/RELEASE_ARCHIVE_MAP.md` | Root compatibility map. |

## Core docs layout

Core Base1 architecture and public-facing design documents remain under `base1/` unless a future compatibility plan says otherwise.

```text
base1/README.md
base1/SECURITY_MODEL.md
base1/HARDWARE_TARGETS.md
base1/PHASE1_COMPATIBILITY.md
base1/ROADMAP.md
base1/NETWORK_LOCKDOWN_DRY_RUN.md
base1/config/base1-secure-profile.toml
base1/systemd/phase1-base1.service
```

## Organization docs layout

Organization, inventory, readiness, and validation-control docs live under `docs/base1/`.

```text
docs/base1/README.md
docs/base1/DOCUMENTATION_MAP.md
docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md
docs/base1/INVENTORY.md
docs/base1/TEST_INVENTORY.md
docs/base1/MIGRATION_TABLE.md
docs/base1/SCRIPT_COMPATIBILITY_PLAN.md
docs/base1/LINK_CHECK_STRATEGY.md
docs/base1/REORGANIZATION_READINESS.md
docs/base1/RELEASE_ARCHIVE_MAP.md
```

## Release/checkpoint notes layout

Organized release/checkpoint browsing lives under `docs/base1/releases/`.

Root-level checkpoint notes remain compatibility paths.

```text
docs/base1/releases/README.md
docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md
```

## Real-device read-only layout

Real-device read-only evidence and runbooks remain under `docs/base1/real-device/`.

```text
docs/base1/real-device/README.md
docs/base1/real-device/READONLY_VALIDATION_PLAN.md
docs/base1/real-device/READONLY_REPORT_TEMPLATE.md
docs/base1/real-device/READONLY_VALIDATION_BUNDLE_REPORT.md
docs/base1/real-device/RUNBOOK.md
docs/base1/real-device/CHECKLIST.md
docs/base1/real-device/STATUS_SUMMARY.md
docs/base1/real-device/reports/
```

## OS-track design slices

Base1 OS-track design slices remain in `docs/os/` while Phase1 OS-track docs live there.

```text
docs/os/BASE1_IMAGE_BUILDER.md
docs/os/BASE1_INSTALLER_DRY_RUN.md
docs/os/BASE1_RECOVERY_COMMAND.md
docs/os/BASE1_STORAGE_LAYOUT_CHECKER.md
docs/os/BASE1_ROLLBACK_METADATA.md
docs/os/BASE1_DRY_RUN_COMMANDS.md
```

## Script layout

Current script paths remain the stable operator interface.

```text
scripts/base1-*.sh
```

A future internal layout such as `scripts/base1/<group>/` may be added only with wrappers that preserve existing `scripts/base1-*.sh` command paths.

## Test layout

Base1 integration tests remain discoverable by Cargo under `tests/`.

```text
tests/base1_*.rs
tests/quality_base1_*.rs
tests/*base1*.rs
```

Do not move or rename tests unless `docs/base1/TEST_INVENTORY.md` and related tests are updated in the same change.

## Compatibility policy

Compatibility paths remain valid even after organized mirrors are added. In particular:

- Root release/checkpoint notes stay available.
- Existing script paths stay available.
- Existing Cargo integration test discovery stays available.
- Existing public documentation links stay recoverable.

## Validation before any movement

Run:

```bash
sh scripts/quality-check.sh base1-docs
cargo test --all-targets
```

## Non-claims

This layout document does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only defines proposed stable repository paths for safer organization.
