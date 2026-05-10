# Base1 recovery USB emergency shell summary

This summary ties together the read-only emergency shell behavior path for Base1 recovery USB planning.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, launch privileged shells, or modify host trust.

## Target scope

- Firmware profile: Libreboot expected.
- Hardware profile: ThinkPad X200-class expected.
- Bootloader expectation: GRUB first.
- Recovery media: external USB planned.
- Target identity: required before writing.
- Image provenance: required before writing.
- Emergency shell access: must remain available.
- Root/admin boundary: operator-visible.
- Secure Boot: not assumed.
- TPM: not assumed.
- Current maturity: emergency shell reports, validation bundle, design, and read-only documentation.

## Read first

1. `base1/RECOVERY_USB_EMERGENCY_SHELL.md`
2. `base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md`
3. `base1/RECOVERY_USB_IMAGE_SUMMARY.md`
4. `base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md`
5. `base1/RECOVERY_USB_COMMAND_INDEX.md`

## First commands

Run the read-only commands first:

    sh scripts/base1-recovery-usb-emergency-shell-summary.sh
    sh scripts/base1-recovery-usb-emergency-shell-validate.sh
    sh scripts/base1-recovery-usb-emergency-shell-report.sh
    sh scripts/base1-recovery-usb-image-summary.sh
    sh scripts/base1-recovery-usb-image-validate.sh
    sh scripts/base1-recovery-dry-run.sh --dry-run

## Emergency shell behavior fields

The current emergency shell path records:

- Emergency shell entry path.
- Keyboard availability.
- Display readability.
- Root/admin boundary.
- Phase1 auto-launch status.
- Phase1 state path.
- Rollback metadata path.
- Recovery media boot path.
- Network availability.
- Offline recovery capability.
- Log collection path.
- Exit/reboot path.

## Required behavior

Emergency shell access must remain available.

Root/admin boundaries must remain visible.

Automatic recovery must not hide emergency access.

## Still not claimed

This summary does not claim:

- Emergency shell execution readiness.
- Privileged shell launch support.
- USB media writing readiness.
- Bootable Base1 image readiness.
- Destructive installer readiness.
- Automatic recovery readiness.
- Real-hardware recovery completion.
- Real-hardware rollback completion.

## Promotion rule

Recovery USB emergency shell behavior must remain read-only until shell entry, exit, state access, rollback metadata, log collection, storage layout, and actual Libreboot/X200-class boot behavior are verified.
