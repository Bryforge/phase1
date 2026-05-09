# Base1 recovery USB image provenance and checksum design

This design defines the read-only image provenance and checksum verification path for Base1 recovery USB planning.

This document is advisory and read-only. It does not download images, write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Goal

Define how an operator should identify, record, and verify a future recovery USB image before any media-writing command exists.

The image provenance path must keep image origin, checksum expectations, signature expectations, target hardware scope, and operator confirmation visible.

## Target scope

Initial scope:

- Libreboot-backed ThinkPad X200-class systems.
- GRUB-first boot path.
- External USB recovery media.
- Explicit target-device selection already completed.
- No Secure Boot assumption.
- No TPM assumption.
- Keyboard-first and offline-first recovery.
- Read-only previews only.

## Image provenance checklist

Before any future image-writing work, record:

- Image filename.
- Image source URL or local source path.
- Image build commit.
- Image build date.
- Image builder identity.
- Expected SHA256 checksum.
- Observed SHA256 checksum.
- Signature status.
- Signing key identity if signatures exist.
- Target hardware profile.
- Target bootloader expectation.
- Recovery shell availability.
- Rollback metadata compatibility.

## Checksum rule

A future recovery USB writer must refuse to write unless the observed checksum exactly matches the expected checksum.

This design does not implement image downloads, signature verification, or media writing. It only records the future verification requirement.

## Read-only commands

Run first:

    sh scripts/base1-recovery-usb-image-validate.sh
    sh scripts/base1-recovery-usb-image-report.sh
    sh scripts/base1-recovery-usb-target-summary.sh
    sh scripts/base1-recovery-usb-target-validate.sh
    sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
    sh scripts/base1-recovery-usb-hardware-summary.sh

## Guardrails

- Do not download images automatically.
- Do not write USB media.
- Do not run dd automatically.
- Do not partition disks.
- Do not format disks.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not flash firmware.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.
- Do not accept missing checksums.
- Do not accept checksum mismatch.
- Do not accept hidden image provenance.

## Promotion rule

A future recovery USB writer can only follow after image provenance, checksum verification, target identity, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.
