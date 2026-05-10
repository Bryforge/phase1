# Base1 Real-Device Read-Only Validation Index

Status: read-only planning and evidence workflow index
Date: 2026-05-10
Scope: Base1 real-device read-only validation path

## Purpose

This index collects the Base1 real-device read-only validation materials.

The path is intentionally non-mutating and evidence-only.

## Documents

- [Read-only validation plan](READONLY_VALIDATION_PLAN.md)
- [Read-only report template](READONLY_REPORT_TEMPLATE.md)
- [Read-only validation bundle report](READONLY_VALIDATION_BUNDLE_REPORT.md)

## Scripts

- `scripts/base1-real-device-readonly-preview.sh`
- `scripts/base1-real-device-readonly-report.sh`
- `scripts/base1-real-device-readonly-validation-bundle.sh`

## Required Workflow

```text
scripts/base1-real-device-readonly-validation-bundle.sh --dry-run --target /dev/YOUR_TARGET
```

## Guardrails

- Dry-run required
- `/dev/` target required
- No disk writes
- No partitioning
- No formatting
- No installer execution
- No firmware flashing
- No bootloader installation
- No automatic target selection
- No destructive repair commands
- No real-device write path

## Non-Claims

- Not installer-ready
- Not hardware-validated
- Not daily-driver ready
- No destructive disk writes
- No real-device write path
- [Read-only validation runbook](RUNBOOK.md)
- [Read-only validation checklist](CHECKLIST.md)
- [Read-only evidence capture report](reports/2026-05-10-readonly-evidence-capture.md)
