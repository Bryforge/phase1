# Base1 recovery USB emergency shell command index

This index collects the read-only emergency shell behavior documents and commands for Base1 recovery USB planning.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, launch privileged shells, or modify host trust.

## Emergency shell documents

- `base1/RECOVERY_USB_EMERGENCY_SHELL.md` — emergency shell behavior design.
- `base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md` — emergency shell behavior summary.
- `base1/RECOVERY_USB_IMAGE_SUMMARY.md` — image provenance summary.
- `base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md` — image provenance command index.
- `base1/RECOVERY_USB_COMMAND_INDEX.md` — broader recovery USB command index.

## Emergency shell commands

- `scripts/base1-recovery-usb-emergency-shell-summary.sh`
- `scripts/base1-recovery-usb-emergency-shell-validate.sh`
- `scripts/base1-recovery-usb-emergency-shell-report.sh`
- `scripts/base1-recovery-usb-image-summary.sh`
- `scripts/base1-recovery-usb-image-validate.sh`
- `scripts/base1-recovery-dry-run.sh --dry-run`

## Emergency shell fields

An emergency shell preview must keep these fields operator-visible:

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

## Shared guardrails

Every emergency-shell document or command must preserve these rules:

- Report shell launch: no.
- Report writes: no.
- Keep emergency shell access available.
- Keep root/admin boundaries operator-visible.
- Do not launch privileged shells automatically.
- Do not remove emergency shell access.
- Do not hide root/admin boundaries.
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

## Promotion rule

A future recovery USB environment can only promote emergency shell behavior after shell entry, exit, state access, rollback metadata, log collection, storage layout, and actual Libreboot/X200-class boot behavior are verified.


See also: [`RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md`](../RELEASE_BASE1_RECOVERY_USB_EMERGENCY_SHELL_READONLY_V1.md) — recovery USB emergency shell read-only checkpoint release notes.
