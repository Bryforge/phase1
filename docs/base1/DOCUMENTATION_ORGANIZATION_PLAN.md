# Base1 Documentation Organization Plan

Status: proposed organization plan
Scope: Base1 markdown organization only

## Purpose

Define a safer documentation layout for Base1.

This plan exists so future file organization can happen one group at a time with tests and link updates.

## Current Rule

Keep existing Base1 markdown files available unless a later explicit approval says otherwise.

Do not move existing Base1 markdown files unless the same PR updates every link, test, and index reference.

## Preservation Rule

Base1 organization work is preservation-first. Old locations should remain available as legacy, mirrored, or compatibility paths after new canonical locations are added.

If material ever needs to leave the main repository, place it in a separate holding repository first so it remains recoverable.

## Proposed Groups

### Core
- README.md
- DOCUMENTATION_MAP.md
- VALIDATION_RUNBOOK.md
- VALIDATION_REPORT_TEMPLATE.md
- VALIDATION_REPORTS.md

### Real-Device Read-Only
- real-device/README.md
- real-device/READONLY_VALIDATION_PLAN.md
- real-device/READONLY_REPORT_TEMPLATE.md
- real-device/READONLY_VALIDATION_BUNDLE_REPORT.md
- real-device/RUNBOOK.md
- real-device/CHECKLIST.md
- real-device/STATUS_SUMMARY.md
- real-device/reports/*.md

### Future Candidate Folders
- core/
- validation/
- real-device/
- real-device/reports/
- design/
- archive/

## Migration Rules

- Move one document group per PR.
- Keep or update every inbound link in the same PR.
- Add or update tests for every moved document.
- Preserve non-claims and promotion rules.
- Prefer indexes before file movement.
- Keep legacy paths recoverable.

## Non-Claims

- No runtime behavior change.
- Not installer-ready.
- Not hardware-validated.
- Not daily-driver ready.
- No real-device write path.

## Test-guarded boundaries

- without moving files in this PR
- No file moves in this plan
- No destructive disk writes
