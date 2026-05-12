# Base1 validation reports archive

> **Status:** Documentation index.
>
> **Validation:** Linked from the Base1 manual and checked by `scripts/base1-doc-integrity.sh`.
>
> **Non-claims:** This archive does not prove boot readiness, hardware validation, installer readiness, recovery completeness, hardening, or daily-driver readiness by itself.

This page is the organized archive location for Base1 validation reports.

Use this index to keep evidence discoverable when Base1 work moves from design or dry-run status toward preview, validation, or release-facing checkpoints.

## Current reports and templates

- [`VALIDATION_REPORT_TEMPLATE.md`](VALIDATION_REPORT_TEMPLATE.md) — Base1 validation report template.
- [`VALIDATION_RUNBOOK.md`](VALIDATION_RUNBOOK.md) — documentation-only validation runbook.
- [`validation/README.md`](validation/README.md) — validation reports directory.
- [`real-device/READONLY_VALIDATION_BUNDLE_REPORT.md`](real-device/READONLY_VALIDATION_BUNDLE_REPORT.md) — real-device read-only validation bundle report.
- [`real-device/READONLY_REPORT_TEMPLATE.md`](real-device/READONLY_REPORT_TEMPLATE.md) — real-device read-only report template.

## Recording rule

Every future Base1 validation report should record:

- scope;
- target;
- commands run;
- result;
- observations;
- evidence links;
- boundaries;
- promotion recommendation.

Reports must preserve conservative wording. Do not claim boot readiness, installer readiness, hardware validation, recovery completeness, hardening, or daily-driver readiness without matching evidence.

## Non-claims

This archive is an index and evidence destination only. It does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, VM-validated, release-candidate ready, or daily-driver ready.

## Public surface validation — 2026-05-12

- [`validation/BASE1_PUBLIC_SURFACE_VALIDATION_2026_05_12.md`](validation/BASE1_PUBLIC_SURFACE_VALIDATION_2026_05_12.md) — Base1 public-surface, docs-integrity, link-check, test-inventory, security/crypto, quality, Rust, audit, and deny validation report.
