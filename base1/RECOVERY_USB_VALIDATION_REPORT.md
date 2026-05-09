# Base1 recovery USB validation report

This report template records operator-visible validation for Base1 recovery USB planning.

This document is advisory and read-only. Do not store passwords, tokens, private keys, recovery phrases, or personal secrets here.

## Target summary

- Recovery media: external USB planned.
- Firmware profile: Libreboot expected.
- Hardware profile: X200-class expected.
- Bootloader expectation: GRUB first.
- Secure Boot: not assumed.
- TPM: not assumed.
- Emergency shell: required.
- Phase1 state path: /state/phase1 preview.
- Rollback metadata path: /recovery preview.
- Current maturity: read-only planning and dry-runs.

## Validation commands

Run and record whether each command completed:

    sh scripts/base1-recovery-usb-index.sh
    sh scripts/base1-recovery-usb-hardware-report.sh
    sh scripts/base1-recovery-usb-hardware-validate.sh
    sh scripts/base1-recovery-usb-validate.sh
    sh scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example
    sh scripts/base1-libreboot-validate.sh
    sh scripts/base1-grub-recovery-dry-run.sh --dry-run
    sh scripts/base1-recovery-dry-run.sh --dry-run
    sh scripts/base1-storage-layout-dry-run.sh --dry-run --target /dev/example

## Operator confirmations

Record yes/no/status:

- I can identify the intended USB recovery target.
- I understand this checkpoint does not write USB media.
- I can reach the GRUB menu.
- I know the normal boot path.
- I know the recovery boot path.
- I can reach an emergency shell.
- I know where Phase1 state should live.
- I know where rollback metadata should live.
- I understand wireless firmware limitations.
- I understand clock drift risk.
- I understand no firmware, GRUB, /boot, disk, or boot-order mutation should be automatic.

## Guardrails

- Do not write USB media.
- Do not run dd automatically.
- Do not partition disks.
- Do not format disks.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not flash firmware.
- Do not remove emergency shell access.
- Do not store secrets.

## Result

Status: not validated yet.

Notes:

- Add hardware-specific notes here after running read-only checks.


See also: [Recovery USB hardware validation checklist](RECOVERY_USB_HARDWARE_CHECKLIST.md).


See also: [Recovery USB hardware validation summary](RECOVERY_USB_HARDWARE_SUMMARY.md).
