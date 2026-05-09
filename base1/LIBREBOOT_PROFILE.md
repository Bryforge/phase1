# Base1 Libreboot profile

The Base1 Libreboot profile documents the firmware-aware path for running Phase1 on Libreboot-backed hardware.

This profile is documentation-only. It does not change firmware, boot settings, partitions, recovery media, or host trust.

## Goal

Support Libreboot-backed operator laptops as a first-class Base1 target profile without assuming proprietary firmware tools or hidden host authority.

## Intended target

The first Libreboot-friendly target is the ThinkPad X200-class operator laptop profile.

The profile should prioritize:

- Terminal-first operation.
- Keyboard-only recovery.
- Offline-first installation notes.
- Conservative display and color output.
- Explicit recovery media.
- No hidden boot changes.
- No host trust escalation from Phase1.

## Firmware boundary

Phase1 and Base1 must not silently modify firmware state.

Libreboot-specific guidance should be treated as operator documentation, not automatic mutation.

## Compatibility assumptions

A Libreboot Base1 profile should avoid assuming:

- Vendor firmware management tools.
- Proprietary firmware update paths.
- Secure Boot availability.
- TPM availability.
- Modern graphical firmware setup screens.
- Network boot availability.
- Automatic bootloader repair.
- Non-GRUB boot compatibility.

## Bootloader expectation

For this profile, treat GRUB as the first expected bootloader path unless a specific Libreboot-backed machine has been verified with another boot path.

Base1 should not assume systemd-boot, EFI-only boot, or automatic bootloader repair for Libreboot-backed X200-class systems.

## Required operator notes

The profile should document:

- Current firmware profile: Libreboot.
- Boot media path.
- Emergency shell path.
- Recovery USB path.
- Disk encryption status.
- Wireless firmware limitations.
- Clock drift risk.
- Display/color fallback.
- Keyboard-only recovery steps.

## Guardrails

- Do not flash firmware.
- Do not change boot order automatically.
- Do not install bootloaders automatically.
- Do not assume Secure Boot.
- Do not assume TPM.
- Do not remove emergency shell access.
- Do not hide host and Phase1 authority boundaries.

## First validation command

```bash
sh scripts/base1-preflight.sh
```

Future Libreboot-aware validation may add read-only firmware notes, but firmware mutation remains explicit operator work.
