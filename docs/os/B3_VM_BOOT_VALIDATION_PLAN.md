# Base1 B3 VM boot validation plan

Status: planning
Scope: evidence needed before Base1 can claim VM boot validation for a named profile

## Purpose

B3 is the next boot-readiness target after B2 dry-run assembly.

B3 means a named virtual-machine profile has been run through an expected boot path and has a written validation report. It does not mean physical hardware support, installer readiness, hardening, release-candidate readiness, or daily-driver readiness.

## Entry gate

B3 validation should not start until the focused B2 test suite has passed locally or in CI.

B2 test bundle:

- [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md)

## Initial profile

Start with:

```text
x86_64-vm-validation
```

This keeps the next validation step away from physical hardware claims.

## Planned dry-run command shape

```bash
sh scripts/base1-b3-vm-validate.sh --dry-run --profile x86_64-vm-validation
```

The first command surface should be dry-run only. Any real VM run should come later with a separate validation report and captured logs.

## Evidence required for B3

A future B3 validation report should include:

- selected VM profile;
- VM runtime used;
- architecture and firmware mode;
- boot artifact identifier;
- command used for the VM run;
- boot result;
- Phase1 launch result;
- emergency fallback result or known limitation;
- captured logs path;
- known limitations;
- explicit non-claims.

## B3 checklist

- [ ] B2 test suite has passed locally or in CI.
- [ ] VM profile is explicit.
- [ ] VM runtime is explicit.
- [ ] Boot artifact is explicit.
- [ ] Boot command is documented.
- [ ] Logs are captured.
- [ ] Phase1 launch result is recorded.
- [ ] Emergency fallback result or limitation is recorded.
- [ ] Known limitations are documented.
- [ ] VM result is not generalized to physical hardware.
- [ ] Non-claims are preserved.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)
- [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md)

## Non-claims

This B3 plan does not make Base1 bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It defines the evidence required for a future named VM boot validation claim.
