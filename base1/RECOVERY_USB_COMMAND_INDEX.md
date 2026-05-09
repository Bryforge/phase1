# Base1 recovery USB command index

This index collects the recovery USB planning docs and read-only commands for Base1 recovery-media work.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Recovery USB documents

- `base1/RECOVERY_USB_DESIGN.md` — recovery USB design and safety contract.
- `base1/LIBREBOOT_DOCS_SUMMARY.md` — Libreboot documentation path.
- `base1/LIBREBOOT_MILESTONE.md` — Libreboot read-only milestone checkpoint.
- `base1/LIBREBOOT_VALIDATION_REPORT.md` — validation report template.
- `RELEASE_BASE1_LIBREBOOT_READONLY_V1.md` — v1 checkpoint release notes.
- `RELEASE_BASE1_LIBREBOOT_READONLY_V1_1.md` — v1.1 patch release notes.

## Recovery USB commands

- `scripts/base1-recovery-usb-index.sh`
- `scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example`
- `scripts/base1-libreboot-validate.sh`
- `scripts/base1-libreboot-report.sh`
- `scripts/base1-grub-recovery-dry-run.sh --dry-run`
- `scripts/base1-recovery-dry-run.sh --dry-run`
- `scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example`

## Shared guardrails

Every recovery USB document or command must preserve these rules:

- Require dry-run mode for target-specific previews.
- Make the target device visible.
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
- Do not remove emergency shell access.

## Operator path

The expected read-only path is:

1. Read the recovery USB design.
2. Run the Libreboot validation bundle.
3. Run the recovery USB dry-run with an explicit target.
4. Confirm emergency shell access.
5. Confirm rollback metadata location.
6. Confirm Phase1 state location.
7. Confirm recovery USB plan without writing media.
