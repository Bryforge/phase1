# Base1 Validation Runbook

> **Status:** Documentation-only runbook.
>
> **Validation:** Lists Base1 documentation checks that can be run locally or in CI.
>
> **Non-claims:** This runbook does not validate boot behavior, hardware behavior, image creation, installer behavior, recovery execution, rollback execution, or daily-driver readiness.

Use this runbook to verify the current Base1 documentation evidence chain.

## Scope

This runbook covers documentation checks only:

- Base1 readiness matrix docs;
- validation report template docs;
- validation reports archive docs;
- documentation-only evidence-chain report docs.

## Safe command set

Run these from the repository root:

```bash
cargo test -p phase1 --test base1_readiness_matrix_docs
cargo test -p phase1 --test base1_validation_report_template_docs
cargo test -p phase1 --test base1_validation_reports_index_docs
cargo test -p phase1 --test base1_docs_evidence_chain_report_docs
```

## Combined local run

```bash
cargo test -p phase1 --test base1_readiness_matrix_docs \
  && cargo test -p phase1 --test base1_validation_report_template_docs \
  && cargo test -p phase1 --test base1_validation_reports_index_docs \
  && cargo test -p phase1 --test base1_docs_evidence_chain_report_docs
```

## Expected result

Each test target should report passing tests.

## What this verifies

These commands verify that:

- `docs/base1/READINESS_MATRIX.md` exists and preserves evidence levels;
- `docs/base1/VALIDATION_REPORT_TEMPLATE.md` exists and preserves required report fields;
- `docs/base1/validation/README.md` exists and links the template and readiness matrix;
- `docs/base1/validation/2026-05-10-docs-evidence-chain.md` exists and preserves its documentation-only boundary.

## What this does not verify

This runbook does not verify:

- a bootable Base1 image;
- hardware validation;
- recovery completion;
- installer readiness;
- rollback execution;
- image creation;
- persistence behavior;
- daily-driver readiness.

## Report guidance

When these checks pass, record the result only as documentation evidence. Do not promote any Base1 track beyond the level supported by the readiness matrix.

Use [`VALIDATION_REPORT_TEMPLATE.md`](VALIDATION_REPORT_TEMPLATE.md) when recording future evidence reports.
