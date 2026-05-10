# Base1 root compatibility map

Status: active compatibility map
Scope: Base1 files that still exist at the repository root for public compatibility

## Purpose

Some Base1 checkpoint notes are now mirrored under `docs/base1/releases/` for cleaner navigation. The original root-level files remain available for existing links, bookmarks, branch notes, and release references.

## Compatibility rule

Root-level checkpoint notes are compatibility paths. The organized docs tree is the preferred navigation path, but the root files remain recoverable and visible.

## Root checkpoint notes

| Root path | Organized mirror |
| --- | --- |
| `RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` |
| `RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` | `docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` |
| `RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md` |
| `RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md` | `docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md` |

## Operator guidance

Use `docs/base1/releases/README.md` for organized browsing.

Use the root files when following older links, release notes, or checkpoint references.

## Integrity check

Run:

```bash
sh scripts/base1-doc-integrity.sh
```

The integrity gate should verify that both compatibility paths and organized mirrors remain present.

## Non-claims

This map only documents repository organization. It does not make Base1 installer-ready, hardware-validated, or daily-driver ready.
