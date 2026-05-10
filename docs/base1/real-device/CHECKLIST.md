# Base1 Real-Device Read-Only Validation Checklist

Status: read-only operator checklist
Date: 2026-05-10
Scope: Base1 real-device read-only validation evidence collection

## Purpose

Provide a short operator checklist before, during, and after Base1 real-device read-only validation.

This checklist does not authorize writes, installation, formatting, partitioning, firmware flashing, bootloader installation, or repair actions.

## Before Running

- Confirm you are on the current `edge/stable` branch.
- Review `docs/base1/real-device/README.md`.
- Review `docs/base1/real-device/RUNBOOK.md`.
- Review `docs/base1/real-device/READONLY_VALIDATION_PLAN.md`.
- Review `docs/base1/real-device/READONLY_REPORT_TEMPLATE.md`.
- Identify the target path manually.
- Confirm the target path starts with `/dev/`.
- Do not use automatic target selection.

## Required Command

```text
scripts/base1-real-device-readonly-validation-bundle.sh --dry-run --target /dev/YOUR_TARGET
```

## During Running

- Keep `--dry-run` enabled.
- Confirm the printed target path matches the intended `/dev/` target.
- Confirm writes remain disabled.
- Confirm mutation remains disabled.
- Confirm installer remains disabled.
- Confirm hardware validation claim remains false.
- Confirm daily-driver claim remains false.

## After Running

- Copy only read-only observations into a report.
- Use `READONLY_REPORT_TEMPLATE.md` for report structure.
- Choose one approved result label.
- Preserve non-claims in the report.
- Do not run any write, repair, installer, formatting, partitioning, firmware, or bootloader command.

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

## Non-Claims

- Not installer-ready
- Not hardware-validated
- Not daily-driver ready
- No destructive disk writes
- No real-device write path
