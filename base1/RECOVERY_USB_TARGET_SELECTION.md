# Base1 recovery USB target-device selection design

This design defines the read-only target-device selection path for Base1 recovery USB planning.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Goal

Define how an operator should identify and confirm a recovery USB target before any future media-writing command exists.

The target-device selection path must make device identity visible, preserve operator confirmation, and prevent accidental writes to the host disk or personal data drives.

## Target scope

Initial scope:

- Libreboot-backed ThinkPad X200-class systems.
- GRUB-first boot path.
- External USB recovery media.
- No Secure Boot assumption.
- No TPM assumption.
- Keyboard-first and offline-first recovery.
- Read-only previews only.

## Device identity checklist

Before selecting a target, record:

- Device path.
- Device model/name.
- Device size.
- Removable status.
- Current mount status.
- Current filesystem labels if visible.
- Whether the device contains data that must be preserved.
- Whether the device is the internal system disk.
- Whether the operator physically labeled the USB device.

## Required confirmation language

A future mutating command must require an explicit confirmation phrase before writing media.

Required phrase:

I understand this will write recovery USB media to the selected device

This design does not implement that mutating command. It only records the future confirmation requirement.

## Read-only commands

Run first:

    sh scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
    sh scripts/base1-recovery-usb-hardware-summary.sh
    sh scripts/base1-recovery-usb-hardware-validate.sh
    sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example
    sh scripts/base1-recovery-usb-index.sh

## Guardrails

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
- Do not allow hidden target selection.
- Do not default to the internal system disk.

## Promotion rule

A future recovery USB writer can only follow after target identity, image provenance, checksum verification, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.


See also: [Recovery USB target selection command index](RECOVERY_USB_TARGET_COMMAND_INDEX.md).
