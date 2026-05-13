# Base1 release archive map

Status: active release archive map
Scope: Base1 release/checkpoint notes archived under `docs/base1/releases/` for public compatibility and organized browsing

## Purpose

Base1 checkpoint and release notes are archived under `docs/base1/releases/` for cleaner navigation and stable public links.

Older root-level checkpoint filenames remain documented here as historical compatibility names, but the repository no longer requires those files to exist at the repository root.

## Compatibility rule

Use the organized archive path for current links, docs, validation, and release references.

The former root filenames remain visible in this map so old references can be resolved manually to their archived paths.

## Archived checkpoint notes

| Former root path | Archived path |
| --- | --- |
| `RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` |
| `RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` |
| `RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md` |

## Operator guidance

Use `docs/base1/releases/README.md` for organized browsing.

Use this map when following older release-note names, branch notes, or checkpoint references.

## Integrity check

Run:

```bash
sh scripts/base1-doc-integrity.sh
```

The integrity gate verifies that the release archive map and organized release archive files are present.

## Non-claims

This map only documents repository organization. It does not make Base1 installer-ready, hardware-validated, or daily-driver ready.
