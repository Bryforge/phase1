# Base1 Real QEMU Boot Promotion Report

Status: promotion evidence recorded
Date: 2026-05-10
Base: edge/stable after PR #247

## Scope

This report records the Base1 real QEMU boot promotion checkpoint after the repeatable real Phase1 initrd builder and QEMU marker validation.

## Evidence Chain

Required evidence:

- docs/base1/validation/2026-05-10-qemu-phase1-marker.md
- docs/base1/validation/2026-05-10-qemu-real-phase1-binary.md
- docs/base1/validation/2026-05-10-real-phase1-initrd-builder.md
- scripts/base1-qemu-boot-check.sh

## Guardrails

- Dry-run behavior does not launch QEMU.
- QEMU execution requires explicit confirmation.
- Bundles outside build/ are refused.
- Missing bundle cases fail safely.
- Serial evidence remains the promotion source.

## Promotion Rule

This checkpoint only promotes the QEMU evidence path for the Base1 preview stack.

It does not promote Phase1 to installer-ready, hardware-validated, or daily-driver status.

## Validated Commands

- cargo fmt --all --check
- cargo test -p phase1 --test base1_qemu_boot_check_script
- cargo test -p phase1 --test base1_qemu_phase1_marker_report_docs
- cargo test -p phase1 --test base1_qemu_real_phase1_binary_report_docs
- cargo test -p phase1 --test smoke

## Non-Claims

- No installer readiness claim.
- No hardware validation claim.
- No daily-driver claim.
- No destructive disk writes.
- No real-device write path.
