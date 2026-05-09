# Base1 Libreboot quickstart

This quickstart is the safe first path for a Libreboot-backed, GRUB-first X200-class Base1 target.

This document is advisory and read-only. It does not flash firmware, change boot order, install GRUB, edit grub.cfg, write to /boot, modify disks, or change host trust.

## Target assumptions

- Firmware: Libreboot.
- Hardware: ThinkPad X200-class.
- Bootloader: GRUB first.
- Secure Boot: not assumed.
- TPM: not assumed.
- Recovery media: external USB recommended.
- Emergency shell: required.
- Phase1 posture: safe mode on by default.

## First safe commands

Run the read-only checks first:

    sh scripts/base1-libreboot-index.sh
    sh scripts/base1-libreboot-checklist.sh
    sh scripts/base1-libreboot-preflight.sh
    sh scripts/base1-grub-recovery-dry-run.sh --dry-run
    sh scripts/base1-libreboot-validate.sh
    sh scripts/base1-libreboot-report.sh

## What to confirm before anything destructive exists

Before future install work, confirm:

- You can reach the GRUB menu.
- You know the normal boot path.
- You know the recovery boot path.
- You can reach an emergency shell.
- You know where Phase1 state should live.
- You know where rollback metadata should live.
- You have or plan recovery USB media.
- You understand wireless firmware limitations.
- You understand clock drift risk.
- You understand Base1 must not mutate firmware or boot settings silently.

## Guardrails

- Do not flash firmware automatically.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not remove emergency shell access.
- Do not assume systemd-boot.
- Do not assume EFI-only boot.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.

## Next safe step

After this quickstart passes, keep development in read-only dry-runs until hardware validation, rollback validation, and recovery validation are complete.
