# Base1 Libreboot operator checklist

This checklist ties together the Libreboot, GRUB, recovery USB, dry-run, and Phase1 launch readiness path for X200-class Base1 systems.

This document is advisory and read-only. It does not flash firmware, change boot order, install GRUB, write to /boot, modify disks, or change host trust.

## Target profile

- Firmware: Libreboot.
- Hardware: ThinkPad X200-class.
- Bootloader: GRUB first.
- Secure Boot: not assumed.
- TPM: not assumed.
- Recovery media: external USB recommended.
- Recovery path: emergency shell required.
- Phase1 posture: safe mode on by default.

## Before install or boot-image work

Confirm:

- Current firmware is Libreboot.
- GRUB menu access is understood.
- Emergency shell access is documented.
- Recovery USB exists or is planned.
- Disk encryption status is known.
- Wireless firmware limitations are known.
- Clock drift risk is known.
- Phase1 state path is known.
- Rollback metadata path is known.

## Required dry-runs

Run these before any future destructive installer work:

    sh scripts/base1-libreboot-checklist.sh
    sh scripts/base1-preflight.sh
    sh scripts/base1-libreboot-preflight.sh
    sh scripts/base1-grub-recovery-dry-run.sh --dry-run
    sh scripts/base1-recovery-dry-run.sh --dry-run
    sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example
    sh scripts/base1-rollback-metadata-dry-run.sh --dry-run
    sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example

## Guardrails

- Do not flash firmware automatically.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not remove emergency shell access.
- Do not assume systemd-boot.
- Do not assume EFI-only boot.
- Do not assume Secure Boot.
- Do not assume TPM.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.

## Ready marker

A Libreboot-backed Base1 target is only ready for later install work when the operator can explain the normal boot path, recovery boot path, Phase1 launch path, rollback path, and emergency shell path without relying on hidden host mutation.
