# Base1 migration table

Status: active migration-planning table
Scope: Base1 documentation, release/checkpoint notes, scripts, tests, and compatibility paths

## Purpose

This table defines proposed organization targets before any broad Base1 reorganization happens.

It is preservation-first. The current path remains available unless explicit future approval says otherwise. Proposed targets are navigation and planning targets, not removal instructions.

## Migration rules

- Keep current paths recoverable.
- Add mirrors or indexes before changing references.
- Preserve root compatibility paths for public checkpoint notes.
- Update tests and integrity gates with each group.
- Do not weaken non-claims, dry-run wording, recovery references, or rollback references.
- Run `sh scripts/quality-check.sh base1-docs` after each organization change.

## Core docs

| Current path | Proposed organized path | Compatibility decision | Status |
| --- | --- | --- | --- |
| `base1/README.md` | keep | canonical | active |
| `base1/SECURITY_MODEL.md` | keep | canonical | active |
| `base1/HARDWARE_TARGETS.md` | keep | canonical | active |
| `base1/PHASE1_COMPATIBILITY.md` | keep | canonical | active |
| `base1/ROADMAP.md` | keep | canonical | active |
| `base1/NETWORK_LOCKDOWN_DRY_RUN.md` | keep | canonical network design | active |
| `base1/config/base1-secure-profile.toml` | keep | canonical config | active |
| `base1/systemd/phase1-base1.service` | keep | canonical service template | active |

## Organization docs

| Current path | Proposed organized path | Compatibility decision | Status |
| --- | --- | --- | --- |
| `docs/base1/README.md` | keep | canonical manual index | active |
| `docs/base1/DOCUMENTATION_MAP.md` | keep | canonical map | active |
| `docs/base1/DOCUMENTATION_ORGANIZATION_PLAN.md` | keep | canonical plan | active |
| `docs/base1/ROOT_COMPATIBILITY_MAP.md` | keep | canonical compatibility map | active |
| `docs/base1/INVENTORY.md` | keep | canonical inventory | active |
| `docs/base1/TEST_INVENTORY.md` | keep | canonical test inventory | active |
| `docs/base1/REORGANIZATION_READINESS.md` | keep | canonical readiness checklist | active |

## Release/checkpoint notes

| Current path | Proposed organized path | Compatibility decision | Status |
| --- | --- | --- | --- |
| `RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` | keep root compatibility path | mirrored |
| `RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` | keep root compatibility path | mirrored |
| `RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md` | keep root compatibility path | mirrored |
| `RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md` | keep root compatibility path | mirrored |
| `RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md` | keep root compatibility path | mirrored |
| `RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md` | keep root compatibility path | mirrored |
| `DEVELOPMENT_CHECKPOINT_BASE1_RECOVERY_USB_READONLY_V1.md` | future `docs/base1/releases/` mirror | keep root compatibility path | pending review |

## Recovery USB docs

| Current path | Proposed organized path | Compatibility decision | Status |
| --- | --- | --- | --- |
| `base1/RECOVERY_USB_DESIGN.md` | keep for now | canonical recovery USB entry | active |
| `base1/RECOVERY_USB_COMMAND_INDEX.md` | keep for now | canonical recovery USB command index | active |
| `base1/RECOVERY_USB_HARDWARE_CHECKLIST.md` | keep for now | compatibility/canonical hybrid | active |
| `base1/RECOVERY_USB_TARGET_SELECTION.md` | keep for now | compatibility/canonical hybrid | active |
| `base1/RECOVERY_USB_IMAGE_PROVENANCE.md` | keep for now | compatibility/canonical hybrid | active |
| `base1/RECOVERY_USB_EMERGENCY_SHELL.md` | keep for now | compatibility/canonical hybrid | active |

Future candidate folder: `docs/base1/recovery-usb/`. No move until links, tests, and compatibility shims are planned.

## Libreboot docs

| Current path | Proposed organized path | Compatibility decision | Status |
| --- | --- | --- | --- |
| `base1/LIBREBOOT_PROFILE.md` | keep for now | canonical Libreboot profile | active |
| `base1/LIBREBOOT_PREFLIGHT.md` | keep for now | canonical Libreboot preflight | active |
| `base1/LIBREBOOT_GRUB_RECOVERY.md` | keep for now | canonical GRUB recovery docs | active |
| `base1/LIBREBOOT_OPERATOR_CHECKLIST.md` | keep for now | canonical operator checklist | active |
| `base1/LIBREBOOT_COMMAND_INDEX.md` | keep for now | canonical command index | active |

Future candidate folder: `docs/base1/libreboot/`. No move until links, tests, and compatibility shims are planned.

## Scripts

| Current path | Proposed organized path | Compatibility decision | Status |
| --- | --- | --- | --- |
| `scripts/base1-doc-integrity.sh` | keep | quality-critical path | active |
| `scripts/base1-preflight.sh` | keep | public command path | active |
| `scripts/base1-*-dry-run.sh` | keep | public command paths | active |
| `scripts/base1-libreboot-*.sh` | keep | public command paths | active |
| `scripts/base1-recovery-usb-*.sh` | keep | public command paths | active |
| `scripts/base1-real-device-readonly-*.sh` | keep | public command paths | active |

Future candidate folder: `scripts/base1/`. No move until wrapper compatibility scripts are planned.

## Tests

| Current path | Proposed organized path | Compatibility decision | Status |
| --- | --- | --- | --- |
| `tests/base1_*.rs` | keep | Cargo integration-test paths | active |
| `tests/quality_base1_*.rs` | keep | Cargo integration-test paths | active |

Future candidate grouping should preserve Cargo test discovery and avoid renames unless the test inventory is updated in the same change.

## Readiness impact

This migration table moves the project closer to full reorganization readiness, but it is not complete enough for broad movement yet.

Remaining blockers:

- Complete repository-wide test listing.
- Compatibility shim plan for any future script path changes.
- Link checker or equivalent validation for moved markdown paths.
- Final quality pass after each planned group.

## Non-claims

This table does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only documents safe organization planning.
