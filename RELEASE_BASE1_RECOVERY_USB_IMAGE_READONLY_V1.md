# Base1 recovery USB image provenance read-only checkpoint v1

This checkpoint captures the read-only image provenance and checksum verification path for Base1 recovery USB planning.

## Status

- Checkpoint branch: checkpoint/base1-recovery-usb-image-readonly-v1
- Checkpoint tag: base1-recovery-usb-image-readonly-v1
- Firmware profile: Libreboot expected
- Hardware profile: ThinkPad X200-class expected
- Bootloader expectation: GRUB first
- Recovery media: external USB planned
- Target identity: required before future writing
- Checksum rule: exact match required before future writing
- Signature status: operator-confirmed
- Secure Boot: not assumed
- TPM: not assumed
- Current maturity: image provenance reports, validation bundle, summaries, command index, and read-only documentation

## Included documents

- base1/RECOVERY_USB_IMAGE_PROVENANCE.md
- base1/RECOVERY_USB_IMAGE_SUMMARY.md
- base1/RECOVERY_USB_IMAGE_COMMAND_INDEX.md
- base1/RECOVERY_USB_TARGET_SUMMARY.md
- base1/RECOVERY_USB_TARGET_COMMAND_INDEX.md
- base1/RECOVERY_USB_COMMAND_INDEX.md

## Included commands

- scripts/base1-recovery-usb-image-summary.sh
- scripts/base1-recovery-usb-image-validate.sh
- scripts/base1-recovery-usb-image-report.sh
- scripts/base1-recovery-usb-target-summary.sh
- scripts/base1-recovery-usb-target-validate.sh
- scripts/base1-recovery-usb-target-dry-run.sh --dry-run --target /dev/example

## Validation

Run:

    cargo test -p phase1 --test base1_recovery_usb_image_command_index_docs
    cargo test -p phase1 --test base1_recovery_usb_image_summary_script
    cargo test -p phase1 --test base1_recovery_usb_image_summary_docs
    cargo test -p phase1 --test base1_recovery_usb_image_validation_bundle
    cargo test -p phase1 --test base1_recovery_usb_image_report_script
    cargo test -p phase1 --test base1_recovery_usb_image_provenance_docs
    cargo test -p phase1 --test base1_foundation

## Guardrails

- Do not download images automatically.
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
- Do not accept missing checksums.
- Do not accept checksum mismatch.
- Do not accept hidden image provenance.

## Non-claims

This checkpoint does not claim:

- Image download readiness.
- Signature verification implementation.
- USB media writing readiness.
- Bootable Base1 image readiness.
- Destructive installer readiness.
- Hidden image provenance safety beyond read-only guardrails.
- Real-hardware recovery completion.
- Real-hardware rollback completion.

## Next milestone

The next milestone should remain read-only unless image provenance, checksum verification, target identity, emergency shell behavior, rollback metadata, storage layout, and actual Libreboot/X200-class boot behavior are verified.
