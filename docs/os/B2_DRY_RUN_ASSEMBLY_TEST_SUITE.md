# Base1 B2 dry-run assembly test suite

Status: validation command bundle
Scope: focused B2 test commands for dry-run assembly planning, script behavior, limitations, validation, output review, and readiness status

## Purpose

This document collects the focused test commands needed to validate the B2 dry-run assembly preview before B2 can be considered complete.

It does not claim that the tests have passed. It records the commands that must pass locally or in CI before the B2 status can be strengthened.

## Primary command under test

```bash
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
```

Expected command boundary:

- dry-run only;
- requires `--dry-run`;
- requires `--profile`;
- accepts only documented profiles;
- reports `writes: no`;
- reports `mutation: no`;
- reports `network: no`;
- does not claim bootability, installer readiness, recovery completion, hardening, VM validation, hardware validation, release-candidate readiness, or daily-driver readiness.

## Focused B2 tests

Run these focused tests for B2:

```bash
cargo test -p phase1 --test b2_dry_run_assembly_plan_docs
cargo test -p phase1 --test base1_b2_assembly_dry_run_script
cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs
cargo test -p phase1 --test b2_dry_run_assembly_validation_docs
cargo test -p phase1 --test b2_dry_run_assembly_output_review_docs
cargo test -p phase1 --test boot_readiness_status_docs
cargo test -p phase1 --test boot_readiness_race_plan_docs
cargo test -p phase1 --test x86_64_boot_support_roadmap_docs
cargo test -p phase1 --test readme_navigation_reorganization_links
```

## Optional broader checks

Run the normal quick gate when available:

```bash
sh scripts/quality-check.sh quick
```

Run the Base1 docs gate when available:

```bash
sh scripts/quality-check.sh base1-docs
```

## Pass criteria

B2 test-suite validation requires:

- every focused B2 test command exits successfully;
- the B2 dry-run assembly script syntax passes;
- the B2 dry-run assembly script requires `--dry-run`;
- the B2 dry-run assembly script requires a supported profile;
- the B2 dry-run assembly script reports `writes: no`;
- B2 docs preserve limitations and non-claims;
- README, OS roadmap, race plan, x86_64 roadmap, and status tracker reflect the B2 boundary;
- no mutating boot, disk, package, or network command pattern appears in the B2 script.

## Current result status

Current result: **not recorded in this document**.

Use this document as the command bundle. Record actual pass/fail evidence in a future validation log or CI status before marking B2 complete.

The latest checked commit status API may return no statuses if CI has not reported or is not attached to the commit. Do not treat missing status checks as a pass.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md`](B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md)
- [`B2_DRY_RUN_ASSEMBLY_VALIDATION.md`](B2_DRY_RUN_ASSEMBLY_VALIDATION.md)
- [`B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md`](B2_DRY_RUN_ASSEMBLY_OUTPUT_REVIEW.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)

## Non-claims

This test-suite command bundle does not make Base1 bootable, installer-ready, recovery-complete, hardened, VM-validated, hardware-validated, release-candidate ready, or daily-driver ready.

It lists the focused validation commands required before B2 can be considered complete.
