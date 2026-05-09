# Base1 recovery USB hardware validation checklist

This checklist records the first real-hardware validation path for Base1 recovery USB planning.

This document is advisory and read-only. It does not write USB media, partition disks, format disks, install GRUB, edit grub.cfg, write to /boot, flash firmware, change boot order, or modify host trust.

## Target

Initial target:

- Firmware profile: Libreboot.
- Hardware profile: ThinkPad X200-class.
- Bootloader expectation: GRUB first.
- Recovery media: external USB.
- Secure Boot: not assumed.
- TPM: not assumed.
- Recovery posture: keyboard-first and offline-first.
- Current maturity: operator checklist only.

## Before hardware validation

Confirm:

- The target machine is the intended Libreboot-backed X200-class system.
- The USB device is physically identified.
- The USB device is not a data drive that needs preservation.
- The operator can reach the GRUB menu.
- The operator can reach a fallback shell or emergency shell.
- The operator knows the normal boot path.
- The operator knows the recovery boot path.

## Read-only validation commands

Run these before any future USB-writing work:

    sh scripts/base1-recovery-usb-hardware-report.sh
    sh scripts/base1-recovery-usb-hardware-validate.sh
    sh scripts/base1-recovery-usb-hardware-checklist.sh
    sh scripts/base1-recovery-usb-index.sh
    sh scripts/base1-recovery-usb-validate.sh
    sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example
    sh scripts/base1-libreboot-validate.sh
    sh scripts/base1-grub-recovery-dry-run.sh --dry-run
    sh scripts/base1-recovery-dry-run.sh --dry-run

## Hardware observations to record

Record yes/no/notes:

- GRUB menu reachable.
- USB boot option visible.
- External USB device physically labeled.
- Keyboard works in boot menu.
- Display is readable in boot/recovery mode.
- Emergency shell path is known.
- Recovery path is known.
- Phase1 state path is known.
- Rollback metadata path is known.
- Wireless limitations are known.
- Clock drift risk is known.
- Power/battery behavior is known.

## Guardrails

- Do not write USB media in this checklist.
- Do not run dd automatically.
- Do not partition disks.
- Do not format disks.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not flash firmware.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.
- Do not claim hardware recovery readiness until the checklist is completed on the target machine.

## Promotion rule

A recovery USB media builder can only follow after this checklist, the validation report, target-device selection, image provenance, checksum verification, emergency shell path, and rollback metadata path are documented and tested.
