# Base1 Libreboot command index

This index collects the Libreboot and GRUB-first Base1 operator documents and read-only scripts.

This document is advisory and read-only. It does not flash firmware, change boot order, install GRUB, edit grub.cfg, write to /boot, modify disks, or change host trust.

## Libreboot documents

- `base1/LIBREBOOT_PROFILE.md` — firmware-aware Base1 profile for Libreboot-backed X200-class systems.
- `base1/LIBREBOOT_PREFLIGHT.md` — read-only Libreboot and GRUB-first preflight notes.
- `base1/LIBREBOOT_GRUB_RECOVERY.md` — GRUB-first recovery notes.
- `base1/LIBREBOOT_OPERATOR_CHECKLIST.md` — operator readiness checklist.

## Libreboot scripts

- `scripts/base1-libreboot-preflight.sh`
- `scripts/base1-grub-recovery-dry-run.sh --dry-run`
- `scripts/base1-libreboot-checklist.sh`

## Related Base1 dry-runs

- `scripts/base1-recovery-dry-run.sh --dry-run`
- `scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example`
- `scripts/base1-rollback-metadata-dry-run.sh --dry-run`
- `scripts/base1-install-dry-run.sh --dry-run --target /dev/example`

## Shared guardrails

Every Libreboot-facing Base1 command or document must preserve these rules:

- Do not flash firmware.
- Do not change boot order.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not write to disk.
- Do not assume systemd-boot.
- Do not assume EFI-only boot.
- Do not assume Secure Boot.
- Do not assume TPM.
- Do not hide recovery uncertainty.
- Do not remove emergency shell access.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.

## Operator path

The expected path is:

1. Read the Libreboot profile.
2. Read the Libreboot preflight notes.
3. Read the GRUB recovery notes.
4. Run the Libreboot checklist command.
5. Run the read-only Libreboot preflight command.
6. Run the GRUB recovery dry-run command.
7. Run the broader Base1 dry-run commands.
8. Confirm normal boot, recovery boot, Phase1 launch, rollback, and emergency shell paths.
