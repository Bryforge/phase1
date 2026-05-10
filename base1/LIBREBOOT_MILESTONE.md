# Base1 Libreboot milestone checkpoint

This checkpoint records the current Libreboot-backed, GRUB-first Base1 validation track.

The milestone is read-only. It does not flash firmware, change boot order, install GRUB, edit grub.cfg, write to /boot, modify disks, or change host trust.

## Status

- Firmware profile: Libreboot documented.
- Hardware profile: X200-class documented.
- Bootloader expectation: GRUB first documented.
- Secure Boot: not assumed.
- TPM: not assumed.
- Recovery media: external USB recommended.
- Emergency shell: required.
- Phase1 posture: safe mode default.
- Current maturity: documentation and read-only scripts.

## Completed read-only surfaces

Documents:

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

## Validation command set

Run:

    cargo test -p phase1 --test base1_libreboot_docs_script
    cargo test -p phase1 --test base1_libreboot_docs_summary_docs
    cargo test -p phase1 --test base1_libreboot_quickstart_docs
    cargo test -p phase1 --test base1_libreboot_command_index_docs
    cargo test -p phase1 --test base1_libreboot_profile_docs
    cargo test -p phase1 --test base1_libreboot_preflight_docs
    cargo test -p phase1 --test base1_libreboot_preflight_script
    cargo test -p phase1 --test base1_libreboot_grub_recovery_docs
    cargo test -p phase1 --test base1_grub_recovery_dry_run_script
    cargo test -p phase1 --test base1_libreboot_operator_checklist_docs
    cargo test -p phase1 --test base1_libreboot_checklist_script
    cargo test -p phase1 --test base1_libreboot_validation_bundle
    cargo test -p phase1 --test base1_libreboot_validation_report_docs
    cargo test -p phase1 --test base1_libreboot_report_script

## Not yet claimed

This milestone does not claim:

- Bootable Base1 image readiness.
- Daily-driver readiness.
- Destructive installer readiness.
- Automatic GRUB repair.
- Hardware recovery validation.
- Rollback validation on real hardware.
- Firmware mutation support.

## Promotion rule

The next milestone should stay read-only unless recovery USB behavior, emergency shell access, rollback metadata, storage layout, and GRUB boot behavior have been validated on the target Libreboot system.

See also: [RELEASE_BASE1_LIBREBOOT_READONLY_V1.md](../docs/base1/releases/RELEASE_BASE1_LIBREBOOT_READONLY_V1.md).
