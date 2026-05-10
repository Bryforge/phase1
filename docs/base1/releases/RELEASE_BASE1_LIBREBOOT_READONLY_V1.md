# Base1 Libreboot read-only validation checkpoint v1

This checkpoint captures the first Libreboot-backed, GRUB-first Base1 validation track.

## Status

- Checkpoint branch: checkpoint/base1-libreboot-readonly-v1
- Checkpoint tag: base1-libreboot-readonly-v1
- Firmware profile: Libreboot
- Hardware profile: ThinkPad X200-class
- Bootloader expectation: GRUB first
- Secure Boot: not assumed
- TPM: not assumed
- Current maturity: documentation and read-only scripts

## What is included

Documents:

- base1/LIBREBOOT_MILESTONE.md
- base1/LIBREBOOT_DOCS_SUMMARY.md
- base1/LIBREBOOT_QUICKSTART.md
- base1/LIBREBOOT_COMMAND_INDEX.md
- base1/LIBREBOOT_PROFILE.md
- base1/LIBREBOOT_PREFLIGHT.md
- base1/LIBREBOOT_GRUB_RECOVERY.md
- base1/LIBREBOOT_OPERATOR_CHECKLIST.md
- base1/LIBREBOOT_VALIDATION_REPORT.md

Scripts:

- scripts/base1-libreboot-milestone.sh
- scripts/base1-libreboot-docs.sh
- scripts/base1-libreboot-index.sh
- scripts/base1-libreboot-checklist.sh
- scripts/base1-libreboot-preflight.sh
- scripts/base1-grub-recovery-dry-run.sh --dry-run
- scripts/base1-libreboot-validate.sh
- scripts/base1-libreboot-report.sh

## Validation

Run:

    cargo test -p phase1 --test base1_libreboot_milestone_script
    cargo test -p phase1 --test base1_libreboot_milestone_docs
    cargo test -p phase1 --test base1_libreboot_docs_script
    cargo test -p phase1 --test base1_libreboot_docs_summary_docs
    cargo test -p phase1 --test base1_libreboot_quickstart_docs
    cargo test -p phase1 --test base1_libreboot_command_index_docs
    cargo test -p phase1 --test base1_libreboot_validation_report_docs
    cargo test -p phase1 --test base1_foundation

## Non-claims

This checkpoint does not claim:

- Bootable Base1 image readiness.
- Daily-driver readiness.
- Destructive installer readiness.
- Automatic GRUB repair.
- Firmware mutation support.
- Hardware recovery validation.
- Rollback validation on real hardware.

## Guardrails

- Do not flash firmware automatically.
- Do not install GRUB automatically.
- Do not edit grub.cfg automatically.
- Do not write to /boot.
- Do not change boot order.
- Do not remove emergency shell access.
- Do not assume systemd-boot.
- Do not assume EFI-only boot.
- Do not store secrets.

## Next milestone

The next milestone should remain read-only unless recovery USB behavior, emergency shell access, rollback metadata, storage layout, and GRUB boot behavior have been validated on the target Libreboot system.
