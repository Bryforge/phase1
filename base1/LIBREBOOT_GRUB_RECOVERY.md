# Base1 Libreboot GRUB recovery notes

These notes define the first recovery guidance for Libreboot-backed, GRUB-first Base1 systems.

This document is advisory and read-only. It does not install GRUB, edit GRUB configuration, write to /boot, change boot order, flash firmware, or modify disks.

## Goal

Give operators a clear recovery path for Libreboot-backed X200-class systems where GRUB is the expected first bootloader.

## Expected profile

- Firmware profile: Libreboot.
- Hardware profile: ThinkPad X200-class.
- Bootloader path: GRUB first.
- Recovery access: emergency shell required.
- Recovery media: external USB recommended.
- Secure Boot: not assumed.
- TPM: not assumed.
- Host trust escalation: not allowed from Phase1.

## Operator recovery checklist

Before treating a Libreboot system as recoverable, document:

- How to reach the GRUB menu.
- Which entry starts the normal Base1 path.
- Which entry starts emergency recovery.
- Where recovery USB media is stored.
- Whether disk encryption is enabled.
- Where Phase1 state lives.
- Where rollback metadata lives.
- How to reach a host shell without Phase1 auto-launch.
- How to restore Phase1 state from backup.

Do not store passwords, recovery phrases, private keys, tokens, or personal secrets in recovery notes.

## Guardrails

- Do not run grub-install automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not flash firmware.
- Do not change boot order.
- Do not assume systemd-boot.
- Do not assume EFI-only boot.
- Do not remove emergency shell access.
- Do not hide boot uncertainty from the operator.

## Related read-only checks

    sh scripts/base1-libreboot-preflight.sh
    sh scripts/base1-grub-recovery-dry-run.sh --dry-run
    sh scripts/base1-recovery-dry-run.sh --dry-run
    sh scripts/base1-rollback-metadata-dry-run.sh --dry-run


See also: [Libreboot operator checklist](LIBREBOOT_OPERATOR_CHECKLIST.md).
