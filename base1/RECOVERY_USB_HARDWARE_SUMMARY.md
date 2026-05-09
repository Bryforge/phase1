# Base1 recovery USB hardware validation summary

This summary ties together the read-only recovery USB hardware validation path for Base1.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Target

- Firmware profile: Libreboot expected.
- Hardware profile: ThinkPad X200-class expected.
- Bootloader expectation: GRUB first.
- Recovery media: external USB planned.
- Secure Boot: not assumed.
- TPM: not assumed.
- Recovery posture: keyboard-first and offline-first.
- Current maturity: documentation, checklist, reports, and read-only dry-runs.

## Read first

1. `base1/RECOVERY_USB_DESIGN.md`
2. `base1/RECOVERY_USB_COMMAND_INDEX.md`
3. `base1/RECOVERY_USB_HARDWARE_CHECKLIST.md`
4. `base1/RECOVERY_USB_VALIDATION_REPORT.md`
5. `base1/LIBREBOOT_MILESTONE.md`

## First commands

Run the read-only commands first:

    sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
    sh scripts/base1-recovery-usb-hardware-summary.sh
    sh scripts/base1-recovery-usb-index.sh
    sh scripts/base1-recovery-usb-hardware-checklist.sh
    sh scripts/base1-recovery-usb-hardware-validate.sh
    sh scripts/base1-recovery-usb-hardware-report.sh
    sh scripts/base1-recovery-usb-validate.sh
    sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example

## Hardware observations

The current hardware path records:

- GRUB menu reachability.
- USB boot option visibility.
- External USB device labeling.
- Keyboard behavior in boot menu.
- Display readability in recovery mode.
- Emergency shell path.
- Normal boot path.
- Recovery boot path.
- Phase1 state path.
- Rollback metadata path.
- Wireless limitations.
- Clock drift risk.
- Power and battery behavior.

## Still not claimed

This summary does not claim:

- USB media writing readiness.
- Bootable Base1 image readiness.
- Destructive installer readiness.
- Daily-driver readiness.
- Automatic GRUB repair.
- Real-hardware recovery completion.
- Real-hardware rollback completion.

## Promotion rule

Recovery USB work must remain read-only until target-device selection, image provenance, checksum verification, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.


See also: [RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md](../RELEASE_BASE1_RECOVERY_USB_HARDWARE_READONLY_V1.md).


See also: [Recovery USB target-device selection design](RECOVERY_USB_TARGET_SELECTION.md).
