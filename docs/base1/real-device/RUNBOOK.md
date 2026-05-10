# Base1 Real-Device Read-Only Validation Runbook

Status: read-only workflow runbook
Date: 2026-05-10
Scope: Base1 real-device read-only validation execution

## Purpose

Define the safe operator flow for collecting read-only real-device validation evidence.

This runbook does not authorize writes, installation, formatting, partitioning, firmware flashing, bootloader installation, or repair actions.

## Preconditions

- Work from the current `edge/stable` branch.
- Review the read-only validation plan.
- Review the read-only report template.
- Review the real-device read-only index.
- Identify the candidate target path manually.
- Do not rely on automatic target selection.

## Required Command

```text
scripts/base1-real-device-readonly-validation-bundle.sh --dry-run --target /dev/YOUR_TARGET
```

## Operator Steps

1. Confirm the target path is a `/dev/` path.
2. Run the validation bundle with `--dry-run`.
3. Review the printed device identity information.
4. Copy relevant read-only output into a report based on `READONLY_REPORT_TEMPLATE.md`.
5. Label the result as one of the approved result labels.
6. Do not perform any write action from this workflow.

## Approved Result Labels

- `read-only-observed`
- `blocked-before-device-access`
- `operator-aborted`
- `needs-follow-up`

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

## Promotion Rule

This runbook may only support read-only real-device evidence collection.

It does not promote Phase1 to installer-ready, hardware-validated, or daily-driver status.

## Non-Claims

- Not installer-ready
- Not hardware-validated
- Not daily-driver ready
- No destructive disk writes
- No real-device write path
