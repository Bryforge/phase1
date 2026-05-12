# Base1 recovery USB hardware read-only checkpoint v1

This checkpoint captures the read-only hardware validation path for Base1 recovery USB planning.

## Status

- Checkpoint branch: checkpoint/base1-recovery-usb-hardware-readonly-v1
- Checkpoint tag: base1-recovery-usb-hardware-readonly-v1
- Firmware profile: Libreboot expected
- Hardware profile: ThinkPad X200-class expected
- Bootloader expectation: GRUB first
- Recovery media: external USB planned
- Secure Boot: not assumed
- TPM: not assumed
- Current maturity: documentation, checklist, reports, and read-only dry-runs

## Included documents

- base1/RECOVERY_USB_DESIGN.md
- base1/RECOVERY_USB_COMMAND_INDEX.md
- base1/RECOVERY_USB_HARDWARE_CHECKLIST.md
- base1/RECOVERY_USB_VALIDATION_REPORT.md
- base1/RECOVERY_USB_HARDWARE_SUMMARY.md
- base1/LIBREBOOT_MILESTONE.md

## Included commands

- scripts/base1-recovery-usb-index.sh
- scripts/base1-recovery-usb-dry-run.sh --dry-run --target /dev/example
- scripts/base1-recovery-usb-validate.sh
- scripts/base1-recovery-usb-hardware-checklist.sh
- scripts/base1-recovery-usb-hardware-validate.sh
- scripts/base1-recovery-usb-hardware-report.sh
- scripts/base1-recovery-usb-hardware-summary.sh

## Validation

Run:

    cargo test -p phase1 --test base1_recovery_usb_hardware_summary_script
    cargo test -p phase1 --test base1_recovery_usb_hardware_summary_docs
    cargo test -p phase1 --test base1_recovery_usb_hardware_report_script
    cargo test -p phase1 --test base1_recovery_usb_hardware_validation_bundle
    cargo test -p phase1 --test base1_recovery_usb_hardware_checklist_script
    cargo test -p phase1 --test base1_recovery_usb_hardware_checklist_docs
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

## Non-claims

This checkpoint does not claim:

- USB media writing readiness.
- Bootable Base1 image readiness.
- Destructive installer readiness.
- Daily-driver readiness.
- Automatic GRUB repair.
- Real-hardware recovery completion.
- Real-hardware rollback completion.

## Next milestone

The next milestone should remain read-only unless the target USB device, image provenance, checksum verification, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.
