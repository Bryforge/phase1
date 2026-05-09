# Base1 recovery USB target selection command index

This index collects the read-only target-device selection documents and commands for Base1 recovery USB planning.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Target selection documents

- `base1/RECOVERY_USB_TARGET_SELECTION.md` — target-device selection design.
- `base1/RECOVERY_USB_TARGET_SUMMARY.md` — Recovery USB target selection summary.
- `base1/RECOVERY_USB_HARDWARE_SUMMARY.md` — recovery USB hardware validation summary.
- `base1/RECOVERY_USB_HARDWARE_CHECKLIST.md` — hardware observation checklist.
- `base1/RECOVERY_USB_COMMAND_INDEX.md` — broader recovery USB command index.
- `base1/RECOVERY_USB_VALIDATION_REPORT.md` — validation report template.

## Target selection commands

- `scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example`
- `scripts/base1-recovery-usb-target-validate.sh`
- `scripts/base1-recovery-usb-target-report.sh`
- `scripts/base1-recovery-usb-hardware-summary.sh`
- `scripts/base1-recovery-usb-hardware-checklist.sh`
- `scripts/base1-recovery-usb-hardware-validate.sh`
- `scripts/base1-recovery-usb-hardware-report.sh`
- `scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example`

## Target identity fields

A target selection preview must keep these fields operator-visible:

- Device path.
- Device model/name.
- Device size.
- Removable status.
- Current mount status.
- Filesystem labels if visible.
- Data preservation status.
- Internal disk status.
- Physical USB label status.
- Confirmation status.

## Shared guardrails

Every target-selection document or command must preserve these rules:

- Require dry-run mode for target-specific previews.
- Require an explicit target device path.
- Make target identity visible.
- Report writes: no.
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
- Do not allow hidden target selection.
- Do not default to the internal system disk.

## Promotion rule

A future recovery USB writer can only follow after target identity, image provenance, checksum verification, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.
