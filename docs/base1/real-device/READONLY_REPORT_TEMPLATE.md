# Base1 Real-Device Read-Only Validation Report Template

Status: template only
Date: YYYY-MM-DD
Scope: read-only real-device evidence capture

## Target Identity

- Operator:
- Device path:
- Device model:
- Device serial:
- Device size:
- Transport:
- Host platform:
- Collection date:

## Evidence Source

Use only read-only collection paths.

Allowed examples:

- `scripts/base1-real-device-readonly-preview.sh --dry-run --target /dev/<device>`
- read-only `lsblk` identity output
- read-only `diskutil info` identity output
- operator-entered boot environment notes
- QEMU evidence references

## Read-Only Observations

- Boot environment:
- Firmware/platform notes:
- Storage layout notes:
- Device identity notes:
- QEMU evidence reference:

## Guardrails Confirmed

- No disk writes
- No partitioning
- No formatting
- No installer execution
- No firmware flashing
- No bootloader installation
- No automatic target selection
- No destructive repair commands

## Result Label

Choose one:

- `read-only-observed`
- `blocked-before-device-access`
- `operator-aborted`
- `needs-follow-up`

## Promotion Rule

This report may only promote read-only real-device observation evidence.

It does not promote Phase1 to installer-ready, hardware-validated, or daily-driver status.

## Non-Claims

- Not installer-ready
- Not hardware-validated
- Not daily-driver ready
- No destructive disk writes
- No real-device write path
