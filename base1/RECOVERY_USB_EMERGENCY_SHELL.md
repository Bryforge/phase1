# Base1 recovery USB emergency shell behavior design

This design defines the read-only emergency shell behavior path for Base1 recovery USB planning.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, launch privileged shells, or modify host trust.

## Goal

Define how an operator should reason about emergency shell behavior before any recovery USB media-writing or boot-time recovery automation exists.

The emergency shell path must keep fallback access visible, preserve operator control, and avoid hiding host/Base1 authority boundaries.

## Target scope

Initial scope:

- Libreboot-backed ThinkPad X200-class systems.
- GRUB-first boot path.
- External USB recovery media.
- Target identity already visible.
- Image provenance and checksum expectations already visible.
- No Secure Boot assumption.
- No TPM assumption.
- Keyboard-first and offline-first recovery.
- Read-only previews only.

## Emergency shell checklist

Before any future recovery automation, record:

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

A future recovery environment must not remove emergency shell access.

The shell path should be explicit, operator-visible, and documented before any automatic recovery flow can be promoted.

This design does not launch a shell, grant privileges, edit boot settings, or change recovery behavior.

## Read-only commands

Run first:

    sh scripts/base1-recovery-usb-emergency-shell-summary.sh
    sh scripts/base1-recovery-usb-emergency-shell-validate.sh
    sh scripts/base1-recovery-usb-emergency-shell-report.sh
    sh scripts/base1-recovery-usb-image-summary.sh
    sh scripts/base1-recovery-usb-image-validate.sh
    sh scripts/base1-recovery-usb-target-summary.sh
    sh scripts/base1-recovery-usb-hardware-summary.sh
    sh scripts/base1-recovery-dry-run.sh --dry-run

## Guardrails

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
- Do not change boot order.
- Do not flash firmware.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.

## Promotion rule

A future recovery USB environment can only promote emergency shell behavior after shell entry, exit, state access, rollback metadata, log collection, storage layout, and actual Libreboot/X200-class boot behavior are verified.


See also: [Recovery USB emergency shell summary](RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md).


See also: [Recovery USB emergency shell command index](RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md).
