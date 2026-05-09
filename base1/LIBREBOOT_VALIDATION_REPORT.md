# Base1 Libreboot validation report

This report template records operator-visible validation for a Libreboot-backed, GRUB-first Base1 target.

This document is advisory and read-only. Do not store passwords, tokens, private keys, recovery phrases, or personal secrets here.

## Target summary

- Firmware profile: Libreboot.
- Hardware profile: X200-class.
- Bootloader expectation: GRUB first.
- Secure Boot: not assumed.
- TPM: not assumed.
- Recovery media: external USB recommended.
- Emergency shell: required.
- Phase1 posture: safe mode default.

## Validation commands

Run and record whether each command completed:

    sh scripts/base1-libreboot-validate.sh
    sh scripts/base1-libreboot-index.sh
    sh scripts/base1-libreboot-checklist.sh
    sh scripts/base1-libreboot-preflight.sh
    sh scripts/base1-grub-recovery-dry-run.sh --dry-run

## Operator confirmations

Record yes/no/status:

- I can reach the GRUB menu.
- I know the normal boot path.
- I know the recovery boot path.
- I can reach an emergency shell.
- I know where Phase1 state should live.
- I know where rollback metadata should live.
- I have or plan recovery USB media.
- I understand wireless firmware limitations.
- I understand clock drift risk.
- I understand no firmware or boot mutation should be automatic.

## Guardrails

- Do not flash firmware.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not remove emergency shell access.
- Do not assume systemd-boot.
- Do not assume EFI-only boot.
- Do not store secrets.

## Result

Status: not validated yet.

Notes:

- Add hardware-specific notes here after running read-only checks.
