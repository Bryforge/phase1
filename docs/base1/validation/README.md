# Base1 Validation Reports

> **Status:** Report archive index.
>
> **Validation:** Use with `../VALIDATION_REPORT_TEMPLATE.md` and `../READINESS_MATRIX.md`.
>
> **Non-claims:** This index does not claim Base1 is bootable, daily-driver ready, installer-ready, recovery-complete, or validated on real hardware.

This folder is the home for future Base1 validation reports.

Reports should be copied from [`../VALIDATION_REPORT_TEMPLATE.md`](../VALIDATION_REPORT_TEMPLATE.md) and should only strengthen a Base1 readiness level when the linked evidence supports it.

## Report naming

Use descriptive, date-prefixed names:

```text
YYYY-MM-DD-track-target-summary.md
```

Examples:

```text
2026-05-10-recovery-shell-docs-dry-run.md
2026-05-10-image-provenance-checksum-preview.md
2026-05-10-target-identity-x200-read-only.md
```

## Required report fields

Every report should include:

- report metadata;
- evidence level;
- target summary;
- commands or checks run;
- result;
- observations;
- evidence links;
- boundaries and non-claims;
- promotion recommendation;
- follow-up work.

## Evidence level rule

Use the weakest accurate level from [`../READINESS_MATRIX.md`](../READINESS_MATRIX.md):

```text
Roadmap -> Design -> Dry-run -> Preview -> Validated
```

## Current reports

- [`2026-05-10-docs-evidence-chain.md`](2026-05-10-docs-evidence-chain.md) — documentation-only Base1 evidence-chain report.
- [`2026-05-10-preview-stack.md`](2026-05-10-preview-stack.md) — safe Base1 preview-stack mechanics evidence report.

Add reports only when there is evidence to preserve.
- [`2026-05-10-qemu-phase1-marker.md`](2026-05-10-qemu-phase1-marker.md)
- [`2026-05-10-qemu-real-phase1-binary.md`](2026-05-10-qemu-real-phase1-binary.md)
