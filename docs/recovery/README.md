# Recovery and Hardware Reader Path

> **Status:** Roadmap, design, and dry-run planning index.
>
> **Validation:** Links to Base1 design docs, future recovery checklists, and validation reports when they exist.
>
> **Non-claims:** Recovery USB behavior, destructive installer behavior, rollback, and real-hardware recovery are not claimed complete unless backed by named hardware validation and release evidence.

This page is the recovery and hardware entry point for **The Phase1 Codex**.

## Recovery reading order

1. Read [`../security/TRUST_MODEL.md`](../security/TRUST_MODEL.md).
2. Read [`../base1/README.md`](../base1/README.md).
3. Review current hardware targets and maturity language before attempting any hardware workflow.
4. Prefer read-only validation and dry-run scripts before any mutation.
5. Do not write images, repartition disks, or alter boot material unless the exact workflow is documented, validated, and intentionally selected.

## Recovery principles

- Preserve a recovery shell path.
- Verify target identity before mutation.
- Verify image provenance before use.
- Prefer read-only validation bundles first.
- Require explicit operator confirmation for mutation.
- Keep rollback metadata operator-readable.
- Record hardware validation by device, date, artifact, and result.

## Hardware and recovery rule

Do not describe a recovery workflow as complete, hardware-validated, or installer-ready unless the page links to a dated validation report and the exact artifact used.
