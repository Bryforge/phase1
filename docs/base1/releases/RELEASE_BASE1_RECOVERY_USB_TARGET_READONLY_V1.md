# Base1 recovery USB target selection read-only checkpoint v1

This checkpoint captures the read-only target-device selection path for Base1 recovery USB planning.

## Status

- Checkpoint branch: checkpoint/base1-recovery-usb-target-readonly-v1
- Checkpoint tag: base1-recovery-usb-target-readonly-v1
- Firmware profile: Libreboot expected
- Hardware profile: ThinkPad X200-class expected
- Bootloader expectation: GRUB first
- Recovery media: external USB planned
- Target selection mode: explicit device path only
- Secure Boot: not assumed
- TPM: not assumed
- Current maturity: target identity previews, reports, validation bundle, summaries, and read-only documentation

## Included documents

- base1/RECOVERY_USB_TARGET_SELECTION.md
- base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md
- base1/RECOVERY_USB_TARGET_SUMMARY.md
- base1/RECOVERY_USB_HARDWARE_SUMMARY.md
- base1/RECOVERY_USB_COMMAND_INDEX.md
- base1/RECOVERY_USB_VALIDATION_REPORT.md

## Included commands

- scripts/base1-recovery-usb-target-summary.sh
- scripts/base1-recovery-usb-target-validate.sh
- scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example
- scripts/base1-recovery-usb-target-report.sh
- scripts/base1-recovery-usb-hardware-summary.sh
- scripts/base1-recovery-usb-hardware-validate.sh

## Validation

Run:

    cargo test -p phase1 --test base1_recovery_usb_target_summary_script
    cargo test -p phase1 --test base1_recovery_usb_target_summary_docs
    cargo test -p phase1 --test base1_recovery_usb_target_validation_bundle
    cargo test -p phase1 --test base1_recovery_usb_target_report_script
    cargo test -p phase1 --test base1_recovery_usb_target_dry_run_script
    cargo test -p phase1 --test base1_recovery_usb_target_command_index_docs
    cargo test -p phase1 --test base1_foundation

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
- Do not store passwords, tokens, private keys, recovery phrases, or personal secrets.
- Do not allow hidden target selection.
- Do not default to the internal system disk.

## Non-claims

This checkpoint does not claim:

- USB media writing readiness.
- Bootable Base1 image readiness.
- Destructive installer readiness.
- Hidden target discovery safety.
- Automatic internal-disk protection beyond read-only guardrails.
- Real-hardware recovery completion.
- Real-hardware rollback completion.

## Next milestone

The next milestone should remain read-only unless target identity, image provenance, checksum verification, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.
