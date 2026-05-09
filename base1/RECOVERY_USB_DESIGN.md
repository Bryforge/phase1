# Base1 recovery USB design

The Base1 recovery USB design defines the first safe recovery-media path for Phase1 OS-track work.

This checkpoint is documentation-only and read-only. It does not create USB media, write images, partition disks, install GRUB, edit grub.cfg, flash firmware, or change boot order.

## Goal

Provide an operator-visible recovery USB plan before any recovery media builder exists.

The recovery USB path should help an operator reach a trusted emergency shell, inspect Phase1 state, inspect rollback metadata, and recover from a failed Base1 boot path without weakening the host boundary.

## Target profiles

Initial target profiles:

- Libreboot-backed ThinkPad X200-class systems.
- GRUB-first boot path.
- No Secure Boot assumption.
- No TPM assumption.
- Keyboard-first recovery.
- Offline-first notes.
- External USB recovery media.

## Recovery USB contract

A Base1 recovery USB must document:

- Firmware profile.
- Hardware profile.
- Bootloader expectation.
- USB boot path.
- Emergency shell path.
- Phase1 state path.
- Rollback metadata path.
- Storage layout preview.
- Network/offline status.
- Known hardware limitations.

## Required dry-runs before media creation

Run read-only checks first:

    sh scripts/base1-libreboot-docs.sh
    sh scripts/base1-libreboot-milestone.sh
    sh scripts/base1-libreboot-validate.sh
    sh scripts/base1-grub-recovery-dry-run.sh --dry-run
    sh scripts/base1-recovery-dry-run.sh --dry-run
    sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example

## Guardrails

- Do not write USB media in this checkpoint.
- Do not run dd automatically.
- Do not partition disks.
- Do not format disks.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not flash firmware.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.
- Do not remove emergency shell access.

## Promotion rule

A future recovery USB builder can only exist after the dry-run path, target selection, image provenance, checksum verification, rollback metadata, and emergency shell path are documented and tested.
