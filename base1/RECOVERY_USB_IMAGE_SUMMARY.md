# Base1 recovery USB image provenance summary

This summary ties together the read-only image provenance and checksum verification path for Base1 recovery USB planning.

This document is advisory and read-only. It does not download images, write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Target scope

- Firmware profile: Libreboot expected.
- Hardware profile: ThinkPad X200-class expected.
- Bootloader expectation: GRUB first.
- Recovery media: external USB planned.
- Target identity: required before writing.
- Checksum rule: exact match required before future writing.
- Signature status: operator-confirmed.
- Secure Boot: not assumed.
- TPM: not assumed.
- Current maturity: image provenance reports, validation bundle, summaries, and read-only documentation.

## Read first

1. `base1/RECOVERY_USB_IMAGE_PROVENANCE.md`
2. `base1/RECOVERY_USB_IMAGE_SUMMARY.md`
3. `base1/RECOVERY_USB_TARGET_SUMMARY.md`
4. `base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md`
5. `base1/RECOVERY_USB_COMMAND_INDEX.md`

## First commands

Run the read-only commands first:

    sh scripts/base1-recovery-usb-emergency-shell-report.sh
    sh scripts/base1-recovery-usb-image-summary.sh
    sh scripts/base1-recovery-usb-image-validate.sh
    sh scripts/base1-recovery-usb-image-report.sh
    sh scripts/base1-recovery-usb-target-summary.sh
    sh scripts/base1-recovery-usb-target-validate.sh
    sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example

## Image provenance fields

The current image provenance path records:

- Image filename.
- Image source URL or local source path.
- Image build commit.
- Image build date.
- Image builder identity.
- Expected SHA256 checksum.
- Observed SHA256 checksum.
- Checksum match status.
- Signature status.
- Signing key identity.
- Target hardware profile.
- Target bootloader expectation.
- Recovery shell availability.
- Rollback metadata compatibility.

## Verification rule

Future media writing must refuse when checksum data is missing or mismatched.

## Still not claimed

This summary does not claim:

- Image download readiness.
- Signature verification implementation.
- USB media writing readiness.
- Bootable Base1 image readiness.
- Destructive installer readiness.
- Hidden image provenance safety beyond read-only guardrails.
- Real-hardware recovery completion.
- Real-hardware rollback completion.

## Promotion rule

Recovery USB image work must remain read-only until image provenance, checksum verification, target identity, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.


See also: [Recovery USB image provenance command index](RECOVERY_USB_IMAGE_COMMAND_INDEX.md).


See also: [RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md](../RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md).


See also: [Recovery USB emergency shell behavior design](RECOVERY_USB_EMERGENCY_SHELL.md).
