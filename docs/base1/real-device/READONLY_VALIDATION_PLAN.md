# Base1 Real-Device Read-Only Validation Plan

Status: planning only  
Date: 2026-05-10  
Scope: read-only real-device validation preparation

## Purpose

Define the safe path for collecting real-device evidence without writing to disks, firmware, boot media, partitions, or attached targets.

## Allowed Evidence

- Device identity summary
- Boot environment notes
- Read-only firmware or platform observations
- Read-only storage layout observations
- QEMU evidence references
- Operator-entered target notes

## Forbidden Actions

- No disk writes
- No partitioning
- No formatting
- No installer execution
- No firmware flashing
- No bootloader installation
- No destructive repair commands
- No automatic target selection

## Required Guardrails

- Target identity must be reviewed before any future device workflow.
- Any future command that touches a target must default to dry-run.
- Any future write path requires a separate PR and explicit confirmation phrase.
- This plan does not claim hardware validation.

## Current Evidence Chain

- Repeatable real Phase1 initrd builder
- QEMU Phase1 marker evidence
- QEMU real Phase1 binary evidence
- Base1 real QEMU boot promotion evidence

## Non-Claims

- Not installer-ready
- Not hardware-validated
- Not daily-driver ready
- No destructive disk writes
- No real-device write path
