# Base1 Real-Device Read-Only Validation Report Template

Status: template only
Scope: read-only real-device observation
Date: YYYY-MM-DD
Operator: TBD
Target identity: TBD

## Purpose

Record real-device observations without writing to disks, firmware, boot media, partitions, or attached targets.

## Required Target Identity

- Device path:
- Model:
- Serial or redacted identifier:
- Size:
- Transport:
- Boot environment:
- Operator notes:

## Allowed Evidence

- Device identity summary
- Read-only boot observations
- Read-only firmware or platform observations
- Read-only storage layout observations
- QEMU evidence references
- Operator-entered notes

## Forbidden Actions

- No disk writes
- No partitioning
- No formatting
- No installer execution
- No firmware flashing
- No bootloader installation
- No destructive repair commands
- No automatic target selection

## Validation Commands

- scripts/base1-real-device-readonly-preview.sh --dry-run --target /dev/...

## Result

Result: observation-only / blocked / incomplete

## Non-Claims

- Not installer-ready
- Not hardware-validated
- Not daily-driver ready
- No destructive disk writes
- No real-device write path
