# Base1 recovery USB image provenance command index

This index collects the read-only image provenance and checksum documents and commands for Base1 recovery USB planning.

This document is advisory and read-only. It does not download images, write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Image provenance documents

- `base1/RECOVERY_USB_IMAGE_PROVENANCE.md` — image provenance and checksum verification design.
- `base1/RECOVERY_USB_IMAGE_SUMMARY.md` — image provenance summary.
- `base1/RECOVERY_USB_TARGET_SUMMARY.md` — target-device selection summary.
- `base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md` — target-device command index.
- `base1/RECOVERY_USB_COMMAND_INDEX.md` — broader recovery USB command index.

## Image provenance commands

- `scripts/base1-recovery-usb-image-summary.sh`
- `scripts/base1-recovery-usb-image-validate.sh`
- `scripts/base1-recovery-usb-image-report.sh`
- `scripts/base1-recovery-usb-target-summary.sh`
- `scripts/base1-recovery-usb-target-validate.sh`
- `scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example`

## Image provenance fields

An image provenance preview must keep these fields operator-visible:

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

## Shared guardrails

Every image-provenance document or command must preserve these rules:

- Report downloads: no.
- Report writes: no.
- Do not download images automatically.
- Do not write USB media.
- Do not run dd automatically.
- Do not partition disks.
- Do not format disks.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not flash firmware.
- Do not change boot order.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.
- Do not accept missing checksums.
- Do not accept checksum mismatch.
- Do not accept hidden image provenance.

## Promotion rule

A future recovery USB writer can only follow after image provenance, checksum verification, target identity, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.


See also: [`RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md`](../docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md) — recovery USB image provenance read-only checkpoint release notes.
- [`docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md`](../docs/base1/releases/RELEASE_BASE1_RECOVERY_USB_IMAGE_READONLY_V1.md) — Recovery USB image read-only checkpoint release notes.
