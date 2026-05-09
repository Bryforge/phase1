# Base1 Libreboot docs summary

This summary ties together the Libreboot-backed, GRUB-first Base1 documentation path for X200-class operator systems.

This document is advisory and read-only. It does not flash firmware, change boot order, install GRUB, edit grub.cfg, write to /boot, modify disks, or change host trust.

## Start here

1. `base1/LIBREBOOT_MILESTONE.md` — Libreboot milestone checkpoint
1. `base1/LIBREBOOT_QUICKSTART.md`
2. `base1/LIBREBOOT_COMMAND_INDEX.md`
3. `base1/LIBREBOOT_PROFILE.md`
4. `base1/LIBREBOOT_PREFLIGHT.md`
5. `base1/LIBREBOOT_GRUB_RECOVERY.md`
6. `base1/LIBREBOOT_OPERATOR_CHECKLIST.md`
7. `base1/LIBREBOOT_VALIDATION_REPORT.md`

## First commands

Run the read-only commands first:

    sh scripts/base1-libreboot-docs.sh
    sh scripts/base1-libreboot-index.sh
    sh scripts/base1-libreboot-checklist.sh
    sh scripts/base1-libreboot-preflight.sh
    sh scripts/base1-grub-recovery-dry-run.sh --dry-run
    sh scripts/base1-libreboot-validate.sh
    sh scripts/base1-libreboot-report.sh

## What this covers

The current Libreboot path covers:

- Libreboot-backed X200-class hardware assumptions.
- GRUB-first bootloader expectation.
- No Secure Boot assumption.
- No TPM assumption.
- Recovery USB recommendation.
- Emergency shell requirement.
- Read-only preflight checks.
- Read-only recovery dry-runs.
- Read-only validation report output.
- Phase1 safe-mode default posture.

## What this does not do

The current Libreboot path does not:

- Flash firmware.
- Install GRUB.
- Edit grub.cfg.
- Write to /boot.
- Modify disks.
- Change boot order.
- Store secrets.
- Claim daily-driver readiness.
- Replace hardware recovery testing.
- Replace rollback validation.

## Promotion rule

Libreboot-backed Base1 work should stay in read-only docs and dry-runs until recovery media, emergency shell access, rollback metadata, storage layout, and hardware-specific boot behavior are validated on the target machine.


See also: [RELEASE_BASE1_LIBREBOOT_READONLY_V1.md](../RELEASE_BASE1_LIBREBOOT_READONLY_V1.md).
