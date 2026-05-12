# Base1 recovery USB target selection summary

This summary ties together the read-only target-device selection path for Base1 recovery USB planning.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Target scope

- Firmware profile: Libreboot expected.
- Hardware profile: ThinkPad X200-class expected.
- Bootloader expectation: GRUB first.
- Recovery media: external USB planned.
- Target selection mode: explicit device path only.
- Secure Boot: not assumed.
- TPM: not assumed.
- Current maturity: target identity previews, reports, validation bundle, and read-only documentation.

## Read first

1. `base1/RECOVERY_USB_TARGET_SELECTION.md`
2. `base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md`
3. `base1/RECOVERY_USB_HARDWARE_SUMMARY.md`
4. `base1/RECOVERY_USB_COMMAND_INDEX.md`
5. `base1/RECOVERY_USB_VALIDATION_REPORT.md`

## First commands

Run the read-only commands first:

    sh scripts/base1-recovery-usb-image-validate.sh
    sh scripts/base1-recovery-usb-image-report.sh
    sh scripts/base1-recovery-usb-target-summary.sh
    sh scripts/base1-recovery-usb-target-validate.sh
    sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
    sh scripts/base1-recovery-usb-target-report.sh
    sh scripts/base1-recovery-usb-hardware-summary.sh
    sh scripts/base1-recovery-usb-hardware-validate.sh

## Target identity fields

The current target-selection path records:

- Device path.
- Device model/name.
- Device size.
- Removable status.
- Current attachment status.
- Filesystem labels if visible.
- Data preservation status.
- Internal disk status.
- Physical USB label status.
- Operator confirmation status.

## Future confirmation phrase

A future mutating recovery USB writer must require this exact operator phrase before writing media:

I understand this will write recovery USB media to the selected device

## Still not claimed

This summary does not claim:

- USB media writing readiness.
- Bootable Base1 image readiness.
- Destructive installer readiness.
- Hidden target discovery safety.
- Automatic internal-disk protection beyond read-only guardrails.
- Real-hardware recovery completion.
- Real-hardware rollback completion.

## Promotion rule

Recovery USB target selection must remain read-only until target identity, image provenance, checksum verification, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.


See also: [RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md](../RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md).


See also: [Recovery USB image provenance and checksum design](RECOVERY_USB_IMAGE_PROVENANCE.md).


See also: [Recovery USB image provenance summary](RECOVERY_USB_IMAGE_SUMMARY.md).
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md`](../docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_TARGET_READONLY_V1.md) — Recovery USB target read-only checkpoint release notes.
