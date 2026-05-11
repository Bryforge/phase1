# Base1 B3 VM boot log capture notes

Status: planning notes
Scope: log evidence expected for future B3 VM boot validation

## Purpose

This note defines what logs and review evidence should exist before a named B3 VM validation claim is made.

B3 log capture is for a named virtual-machine profile only. It is not physical hardware validation and it is not a release-candidate claim.

## Entry gate

Do not treat B3 logs as validation evidence until the B2 focused test suite has passed locally or in CI.

B2 test bundle:

- [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md)

## Planned profile

Initial profile:

```text
x86_64-vm-validation
```

## Planned log categories

A future B3 validation report should capture or reference:

- VM runtime name and version;
- VM profile name;
- architecture mode;
- firmware mode;
- boot artifact identifier;
- command or run configuration used for the VM validation;
- console boot log;
- kernel or init handoff notes;
- Phase1 launch result;
- emergency fallback result or limitation;
- failure mode notes if the VM does not boot;
- known limitations.

## Safe log handling

B3 logs should be reviewed before being committed or published.

Logs must not include:

- private keys;
- tokens;
- passwords;
- recovery codes;
- personal credentials;
- private local file contents;
- unredacted environment variables;
- private network addresses unless intentionally part of a public test fixture;
- personal account identifiers;
- secret-bearing kernel command-line values.

## Suggested future paths

Use explicit paths for future validation evidence, for example:

```text
docs/os/validation/b3-x86_64-vm-validation.md
docs/os/validation/logs/b3-x86_64-vm-validation-console.txt
```

These paths are examples only until a real validation run exists.

## Review checklist

Before B3 is considered complete, confirm:

- [ ] B2 focused test suite passed locally or in CI;
- [ ] VM profile is explicit;
- [ ] VM runtime is explicit;
- [ ] boot artifact is explicit;
- [ ] run configuration is documented;
- [ ] console log is captured or linked;
- [ ] Phase1 launch result is recorded;
- [ ] emergency fallback result or limitation is recorded;
- [ ] logs are reviewed for secrets;
- [ ] limitations are documented;
- [ ] non-claims are preserved.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`B3_VM_BOOT_VALIDATION_PLAN.md`](B3_VM_BOOT_VALIDATION_PLAN.md)
- [`B3_VM_BOOT_VALIDATION_LIMITATIONS.md`](B3_VM_BOOT_VALIDATION_LIMITATIONS.md)
- [`B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md`](B2_DRY_RUN_ASSEMBLY_TEST_SUITE.md)

## Non-claims

This log capture note does not make Base1 bootable on physical hardware, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It defines the log evidence expected for future B3 VM validation.
