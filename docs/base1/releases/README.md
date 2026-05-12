# Base1 release and checkpoint notes

Status: active checkpoint-note index
Scope: Base1 read-only release/checkpoint notes

This directory collects Base1 release and checkpoint notes that used to live at the repository root. Keeping them here reduces root clutter while preserving the same non-claims, guardrails, and validation references.

## Pre-move safety

- [Release/checkpoint pre-move checks](PRE_MOVE_CHECKS.md)

Use the pre-move checks before changing this group. Root checkpoint-note files remain compatibility paths and must not be removed during this phase.

## Libreboot checkpoints

- [Base1 Libreboot read-only validation checkpoint v1](RELEASE_BASE1_LIBREBOOT_READONLY_V1.md)
- [Base1 Libreboot read-only validation checkpoint v1.1](RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md)

## Recovery USB checkpoints

- [Base1 recovery USB hardware read-only checkpoint v1](RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md)
- [Base1 recovery USB target selection read-only checkpoint v1](RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md)
- [Base1 recovery USB image provenance read-only checkpoint v1](RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md)
- [Base1 recovery USB emergency shell read-only checkpoint v1](RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md)

## Reorganization rule

Release/checkpoint notes may move here only when inbound links, tests, integrity checks, link checks, and test-inventory verification are updated in the same change set.

## Non-claims

- Not installer-ready.
- Not hardware-validated.
- Not daily-driver ready.
- No destructive disk writes.
- No real-device write path.
- [`docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md`](RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md) — Libreboot read-only checkpoint v1.1 patch release notes.
Libreboot read-only checkpoint v1.1 release notes
- [`docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md`](RELEASE_BASE1_LIBREBOOT_READONLY_V1.md) — Libreboot read-only checkpoint v1 release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md`](RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md) — Recovery USB emergency shell read-only checkpoint release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md`](RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md) — Recovery USB hardware read-only checkpoint release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md`](RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md) — Recovery USB image read-only checkpoint release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md`](RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md) — Recovery USB target read-only checkpoint release notes.
