# Base1 recovery USB emergency shell read-only checkpoint v1

This checkpoint captures the read-only emergency shell behavior path for Base1 recovery USB planning.

## Status

- Checkpoint branch: checkpoint/base1-recovery-usb-emergency-shell-readonly-v1
- Checkpoint tag: base1-recovery-usb-emergency-shell-readonly-v1
- Firmware profile: Libreboot expected
- Hardware profile: ThinkPad X200-class expected
- Bootloader expectation: GRUB first
- Recovery media: external USB planned
- Target identity: required before future writing
- Image provenance: required before future writing
- Emergency shell access: must remain available
- Root/admin boundary: operator-visible
- Secure Boot: not assumed
- TPM: not assumed
- Current maturity: emergency shell reports, validation bundle, summaries, command index, and read-only documentation

## Included documents

- base1/RECOVERY_USB_EMERGENCY_SHELL.md
- base1/RECOVERY_USB_EMERGENCY_SHELL_SUMMARY.md
- base1/RECOVERY_USB_EMERGENCY_SHELL_COMMAND_INDEX.md
- base1/RECOVERY_USB_IMAGE_SUMMARY.md
- base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md
- base1/RECOVERY_USB_COMMAND_INDEX.md

## Included commands

- scripts/base1-recovery-usb-emergency-shell-summary.sh
- scripts/base1-recovery-usb-emergency-shell-validate.sh
- scripts/base1-recovery-usb-emergency-shell-report.sh
- scripts/base1-recovery-usb-image-summary.sh
- scripts/base1-recovery-usb-image-validate.sh
- scripts/base1-recovery-dry-run.sh --dry-run

## Validation

Run:

    cargo test -p phase1 --test base1_recovery_usb_emergency_shell_command_index_docs
    cargo test -p phase1 --test base1_recovery_usb_emergency_shell_summary_script
    cargo test -p phase1 --test base1_recovery_usb_emergency_shell_summary_docs
    cargo test -p phase1 --test base1_recovery_usb_emergency_shell_validation_bundle
    cargo test -p phase1 --test base1_recovery_usb_emergency_shell_report_script
    cargo test -p phase1 --test base1_recovery_usb_emergency_shell_docs
    cargo test -p phase1 --test base1_foundation

## Guardrails

- Do not launch privileged shells automatically.
- Do not remove emergency shell access.
- Do not hide root/admin boundaries.
- Do not write USB media.
- Do not run dd automatically.
- Do not partition disks.
- Do not format disks.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not flash firmware.
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.

## Non-claims

This checkpoint does not claim:

- Emergency shell execution readiness.
- Privileged shell launch support.
- USB media writing readiness.
- Bootable Base1 image readiness.
- Destructive installer readiness.
- Automatic recovery readiness.
- Real-hardware recovery completion.
- Real-hardware rollback completion.

## Next milestone

The next milestone should remain read-only unless shell entry, exit, state access, rollback metadata, log collection, storage layout, and actual Libreboot/X200-class boot behavior are verified.
