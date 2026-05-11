# Base1 B3 VM boot validation limitations

Status: planning limitations note
Scope: known limits for the future B3 VM boot validation path

## Purpose

This note records what B3 VM boot validation can and cannot prove.

B3 is meant to validate a named virtual-machine profile only. It is not a physical hardware claim and it is not a release claim.

## Planned command shape

```bash
sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation
```

The first B3 command surface should remain dry-run only until B2 validation passes and the B3 report/log workflow is documented.

## What B3 can eventually prove

A completed B3 report can eventually prove, for one named VM profile only:

- the VM profile is explicit;
- the VM runtime is explicit;
- the boot artifact is explicit;
- the boot command is documented;
- logs were captured;
- the observed boot result is recorded;
- the observed Phase1 launch result is recorded;
- the emergency fallback result or limitation is recorded;
- known limitations are documented.

## What B3 cannot prove

A B3 VM result cannot prove:

- physical hardware support;
- installer readiness;
- recovery completion;
- rollback completion;
- hardened status;
- release-candidate readiness;
- daily-driver readiness;
- support for all x86_64 systems;
- support for secure boot, measured boot, TPM, or lockdown modes.

## Required behavior

The B3 path must continue to:

- require B2 validation before B3 claims;
- use a named VM profile;
- keep logs and reports explicit;
- avoid generalizing VM results to physical hardware;
- keep limitations visible;
- preserve non-claims.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`B3_VM_BOOT_VALIDATION_PLAN.md`](B3_VM_BOOT_VALIDATION_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)

## Non-claims

This limitations note does not make Base1 bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It records the boundary for future B3 VM boot validation work.
