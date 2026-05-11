# Base1 B2 dry-run assembly validation

Status: initial validation report
Scope: validation evidence and review checklist for `scripts/base1-b2-assembly-dry-run.sh --dry-run --profile <profile>`

## Purpose

This report records the validation expectations for the first B2 dry-run assembly preview.

It does not claim boot readiness. It records the evidence needed to keep the B2 command bounded to dry-run preview behavior only.

## Command under validation

```bash
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
```

## Test command

```bash
cargo test -p phase1 --test base1_b2_assembly_dry_run_script
```

Related documentation tests:

```bash
cargo test -p phase1 --test b2_dry_run_assembly_plan_docs
cargo test -p phase1 --test b2_dry_run_assembly_limitations_docs
cargo test -p phase1 --test boot_readiness_status_docs
cargo test -p phase1 --test boot_readiness_race_plan_docs
cargo test -p phase1 --test x86_64_boot_support_roadmap_docs
```

## Validation status

| Check | Status | Evidence |
| --- | --- | --- |
| Script exists | Present | `scripts/base1-b2-assembly-dry-run.sh` |
| Script requires `--dry-run` | Guarded by tests | `tests/base1_b2_assembly_dry_run_script.rs` |
| Script requires `--profile` | Guarded by tests | `tests/base1_b2_assembly_dry_run_script.rs` |
| Script rejects unsupported profiles | Guarded by tests | `tests/base1_b2_assembly_dry_run_script.rs` |
| Script accepts planned profiles | Guarded by tests | `tests/base1_b2_assembly_dry_run_script.rs` |
| Script reports `writes: no` | Guarded by tests | `tests/base1_b2_assembly_dry_run_script.rs` |
| Script reports B2 preview sections | Guarded by tests | B1 detection, profile assumptions, image builder, boot handoff, installer, recovery, rollback, validation bundle, limitations |
| Script avoids known mutating command patterns | Guarded by tests | boot-loader, disk, package, and network command patterns are checked |
| B2 plan exists | Present | `docs/os/B2_DRY_RUN_ASSEMBLY_PLAN.md` |
| B2 limitations exist | Present | `docs/os/B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md` |
| B2 status tracker updated | Present | `docs/os/BOOT_READINESS_STATUS.md` |
| CI/local test result | Pending | Run the commands above before claiming B2 complete. |

## Source review checklist

Review `scripts/base1-b2-assembly-dry-run.sh` before B2 completion and confirm:

- [ ] requires `--dry-run`;
- [ ] requires `--profile`;
- [ ] rejects unsupported profiles;
- [ ] prints `writes: no`;
- [ ] prints `mutation: no`;
- [ ] prints `network: no`;
- [ ] does not write images;
- [ ] does not call mutating boot-loader commands;
- [ ] does not call mutating partition/disk commands;
- [ ] does not install packages;
- [ ] does not require network access;
- [ ] does not write to `/boot`, `/etc`, EFI variables, initramfs files, or partitions;
- [ ] reports unknown or unvalidated facts instead of guessing;
- [ ] keeps bootability, installer readiness, recovery completion, hardening, VM validation, hardware validation, and release-candidate claims explicitly unclaimed.

## Output review checklist

Review sample output before B2 completion and confirm it includes:

- [ ] `status: B2 dry-run assembly preview`;
- [ ] `boot_readiness_level: B2`;
- [ ] `writes: no`;
- [ ] `mutation: no`;
- [ ] `network: no`;
- [ ] selected profile;
- [ ] B1 detection summary;
- [ ] profile assumptions;
- [ ] image-builder preview;
- [ ] boot handoff preview;
- [ ] installer preview;
- [ ] recovery preview;
- [ ] rollback preview;
- [ ] validation bundle preview;
- [ ] known limitations;
- [ ] next validation step.

## Known limitations

The B2 command only reports a dry-run assembly preview. It cannot prove image build success, bootability, installer safety, recovery completion, rollback completion, VM validation, hardware validation, hardening, release-candidate readiness, or daily-driver readiness.

See [`B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md`](B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md).

## B2 completion requirement

B2 can be marked complete only after:

- B2 script tests pass in CI or local validation;
- source review confirms no mutation paths;
- output review confirms no secret leakage;
- README, OS roadmap, race plan, x86_64 roadmap, and status tracker all reflect the implemented B2 boundary;
- non-claims remain intact.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_PLAN.md`](B2_DRY_RUN_ASSEMBLY_PLAN.md)
- [`B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md`](B2_DRY_RUN_ASSEMBLY_LIMITATIONS.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)

## Non-claims

This validation report does not make Base1 bootable, installer-ready, recovery-complete, hardened, VM-validated, hardware-validated, release-candidate ready, or daily-driver ready.

It records the validation expectations for the first B2 dry-run assembly script.
