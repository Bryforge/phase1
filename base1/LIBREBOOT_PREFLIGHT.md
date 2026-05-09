# Base1 Libreboot preflight notes

The Base1 Libreboot preflight notes define read-only checks for Libreboot-backed Phase1 operator systems.

This document is advisory. It does not flash firmware, change boot order, install bootloaders, or modify disks.

## Goal

Give operators a clear checklist for confirming that a Libreboot-backed X200-class system can safely run the Base1 path.

## Read-only checks

A Libreboot preflight should report:

- Firmware profile: Libreboot expected.
- Hardware target: X200-class expected.
- Bootloader expectation: GRUB first.
- Secure Boot assumption: not required.
- TPM assumption: not required.
- Emergency shell path: required.
- Recovery USB path: recommended.
- Offline install path: recommended.
- Disk encryption status: operator-confirmed.
- Wireless firmware status: operator-confirmed.

## GRUB expectation

For Libreboot-backed X200-class systems, Base1 should treat GRUB as the first bootloader path until another boot path has been verified on that exact machine.

The preflight should not assume:

- systemd-boot.
- EFI-only boot.
- Secure Boot.
- TPM-backed unlock.
- Automatic bootloader repair.

## Guardrails

- Do not flash firmware.
- Do not change boot order.
- Do not install GRUB automatically.
- Do not write to /boot.
- Do not write to disk.
- Do not weaken recovery access.
- Do not hide firmware or boot uncertainty.

## First command

```bash
sh scripts/base1-preflight.sh
sh scripts/base1-libreboot-preflight.sh
```

Future script support should remain read-only and should print notes instead of mutating firmware or boot settings.
