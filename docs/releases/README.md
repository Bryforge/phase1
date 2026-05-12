# Phase1 release documentation

Status: active release documentation index
Scope: organized release notes, checkpoint notes, compatibility mirrors, and release-facing validation references

## Purpose

This directory is the preferred organized home for release documentation.

Root-level release notes and checkpoint files may remain as compatibility paths when they are public, heavily linked, or referenced by existing workflows. This index provides a minimalist place for organized release documentation without breaking current paths.

## Reorganization policy

Release documentation reorganization is preservation-first.

Rules:

- Do not delete root-level release notes unless a future move map explicitly approves it.
- Prefer adding organized mirrors or indexes before moving release files.
- Keep old path -> new path mappings when files are mirrored or moved.
- Preserve Base1 checkpoint and recovery release references.
- Update tests when release paths become required navigation paths.

## Planned structure

```text
docs/releases/
  README.md
  phase1/
  base1/
  checkpoints/
```

Create subdirectories only when they contain real release documents or indexes.

## Suggested categories

| Category | Purpose |
| --- | --- |
| `phase1/` | Phase1 stable, edge, and milestone release notes. |
| `base1/` | Base1 read-only checkpoint and recovery release notes. |
| `checkpoints/` | Verified checkpoint summaries and compatibility maps. |

## Validation

Before and after release documentation reorganization, run:

```bash
sh scripts/quality-check.sh quick
```

For Base1 release/checkpoint documentation, also run:

```bash
sh scripts/quality-check.sh base1-docs
```

## Non-claims

This index does not move release files by itself, remove release archive paths, publish a release, or prove release readiness.

It creates an organized destination for future release documentation work.
- [`docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md`](../base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md) — Libreboot read-only checkpoint v1.1 patch release notes.
Libreboot read-only checkpoint v1.1 release notes
- [`docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md`](../base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md) — Libreboot read-only checkpoint v1 release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md`](../base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md) — Recovery USB emergency shell read-only checkpoint release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md`](../base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md) — Recovery USB hardware read-only checkpoint release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md`](../base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md) — Recovery USB image read-only checkpoint release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md`](../base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md) — Recovery USB target read-only checkpoint release notes.
