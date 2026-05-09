# Base1 recovery USB read-only development checkpoint v1

This checkpoint records the current Base1 recovery USB planning track after the Libreboot GRUB-first read-only validation milestone.

## Status

- Base track: Base1 foundation.
- Hardware profile: Libreboot-backed ThinkPad X200-class systems.
- Bootloader expectation: GRUB first.
- Recovery media path: external USB planned.
- Current maturity: documentation and read-only dry-runs.
- Edge line after this checkpoint: v5.1.0.
- Stable promotion target: v5.0.0.

## Completed surfaces

Documents:

- `base1/RECOVERY_USB_DESIGN.md`
- `base1/RECOVERY_USB_COMMAND_INDEX.md`
- `base1/RECOVERY_USB_VALIDATION_REPORT.md`
- `base1/LIBREBOOT_MILESTONE.md`
- `base1/LIBREBOOT_DOCS_SUMMARY.md`
- `RELEASE_BASE1_LIBREBOOT_READONLY_V1.md`
- `RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md`

Scripts:

- `scripts/base1-recovery-usb-index.sh`
- `scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example`
- `scripts/base1-recovery-usb-validate.sh`
- `scripts/base1-libreboot-validate.sh`
- `scripts/base1-grub-recovery-dry-run.sh --dry-run`

## Guardrails

- Do not write USB media.
- Do not run `dd` automatically.
- Do not partition or format disks.
- Do not install GRUB automatically.
- Do not edit `grub.cfg` automatically.
- Do not write to `/boot`.
- Do not change boot order.
- Do not flash firmware.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.

## Non-claims

This checkpoint does not claim bootable Base1 image readiness, destructive installer readiness, daily-driver readiness, automatic GRUB repair, real-hardware recovery validation, or real-hardware rollback validation.
