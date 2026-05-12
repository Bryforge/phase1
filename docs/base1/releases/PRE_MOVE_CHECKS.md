# Base1 release/checkpoint pre-move checks

Status: active group-specific pre-move checks
Scope: Base1 release/checkpoint notes and release archive paths

## Purpose

This document defines the group-specific safety checks for the first candidate organization group: Base1 release/checkpoint notes.

It is preservation-first. It does not authorize deletion, root cleanup, or broad movement.

## Candidate group

The candidate group is release/checkpoint notes because organized mirrors already exist under `docs/base1/releases/` while root compatibility files remain present.

## Current compatibility rule

Root-level release/checkpoint files remain compatibility paths.

Organized mirrors live under:

```text
docs/base1/releases/
```

No archived release/checkpoint file should be removed during this phase.

## Required paths

Archived Base1 release paths:

```text
RELEASE_BASE1_LIBREBOOT_READONLY_V1.md
RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md
RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md
RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md
RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md
RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md
```

Organized mirror paths:

```text
docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md
docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md
```

## Required tests before any change

These tests must continue to pass before and after any release/checkpoint note organization change:

```bash
cargo test -p phase1 --test base1_root_compatibility_map_docs
cargo test -p phase1 --test base1_libreboot_release_notes_docs
cargo test -p phase1 --test base1_libreboot_patch_release_notes_docs
cargo test -p phase1 --test base1_recovery_usb_hardware_release_notes_docs
cargo test -p phase1 --test base1_recovery_usb_target_release_notes_docs
cargo test -p phase1 --test base1_recovery_usb_image_release_notes_docs
cargo test -p phase1 --test base1_recovery_usb_emergency_shell_release_notes_docs
cargo test -p phase1 --test base1_link_check_script
cargo test -p phase1 --test quality_base1_docs_gate
```

## Required quality gate

Run:

```bash
sh scripts/quality-check.sh base1-docs
```

This should run the integrity gate, link checker, and test-inventory verifier.

## Blockers

Do not move or mirror this group when:

- Root compatibility files are missing.
- Organized mirror files are missing.
- `docs/base1/RELEASE_ARCHIVE_MAP.md` does not map root paths to mirror paths.
- `docs/base1/releases/README.md` does not index the organized notes.
- Local link checking fails.
- Test inventory verification fails.
- Non-claims are weakened.
- Any change requires deleting root checkpoint files to look clean.

## Non-claims

This pre-move check does not make Base1 installer-ready, hardware-validated, or daily-driver ready. It only defines safety checks for organizing release/checkpoint notes.
