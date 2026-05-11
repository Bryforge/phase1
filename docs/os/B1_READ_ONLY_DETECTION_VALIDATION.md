# Base1 B1 read-only detection validation

Status: initial validation report
Scope: validation evidence and review checklist for `scripts/base1-x86_64-detect.sh --dry-run`

## Purpose

This report records the validation expectations for the first B1 read-only x86_64 detection script.

It does not claim boot readiness. It records the evidence needed to keep the B1 detector bounded to detection-preview behavior only.

## Command under validation

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

## Test command

```bash
cargo test -p phase1 --test base1_x86_64_detect_script
```

Related documentation tests:

```bash
cargo test -p phase1 --test b1_read_only_detection_plan_docs
cargo test -p phase1 --test b1_read_only_detection_limitations_docs
cargo test -p phase1 --test boot_readiness_status_docs
cargo test -p phase1 --test boot_readiness_race_plan_docs
cargo test -p phase1 --test x86_64_boot_support_roadmap_docs
```

## Validation status

| Check | Status | Evidence |
| --- | --- | --- |
| Script exists | Present | `scripts/base1-x86_64-detect.sh` |
| Script requires `--dry-run` | Guarded by tests | `tests/base1_x86_64_detect_script.rs` |
| Script reports `writes: no` | Guarded by tests | `tests/base1_x86_64_detect_script.rs` |
| Script reports core sections | Guarded by tests | architecture, firmware, boot_loader, kernel_cmdline, virtualization, storage_layout, recovery, unknowns |
| Script avoids known mutating command patterns | Guarded by tests | boot-loader, disk, package, and network command patterns are checked |
| B1 plan exists | Present | `docs/os/B1_READ_ONLY_DETECTION_PLAN.md` |
| B1 limitations exist | Present | `docs/os/B1_READ_ONLY_DETECTION_LIMITATIONS.md` |
| B1 status tracker updated | Present | `docs/os/BOOT_READINESS_STATUS.md` |
| CI/local test result | Pending | Run the commands above before claiming B1 complete. |

## Source review checklist

Review `scripts/base1-x86_64-detect.sh` before B1 completion and confirm:

- [ ] requires `--dry-run`;
- [ ] prints `writes: no`;
- [ ] does not call mutating boot-loader commands;
- [ ] does not call mutating partition/disk commands;
- [ ] does not install packages;
- [ ] does not require network access;
- [ ] does not write to `/boot`, `/etc`, EFI variables, initramfs files, or partitions;
- [ ] redacts sensitive-looking kernel command-line values;
- [ ] reports unknown values instead of guessing;
- [ ] exits non-zero when `--dry-run` is missing.

## Output review checklist

Review sample output before B1 completion and confirm it includes:

- [ ] `status: B1 read-only detection preview`;
- [ ] `writes: no`;
- [ ] `mutation: no`;
- [ ] `network: no`;
- [ ] architecture hints;
- [ ] firmware hints;
- [ ] boot-loader hints;
- [ ] kernel command-line availability with conservative redaction;
- [ ] virtualization hints;
- [ ] storage-layout hints;
- [ ] recovery hints;
- [ ] unknown fields;
- [ ] next read-only check.

## Known limitations

The detector only reports hints. It cannot prove bootability, installer readiness, recovery completion, hardening, hardware validation, release-candidate readiness, or daily-driver readiness.

See [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md).

## B1 completion requirement

B1 can be marked complete only after:

- detector tests pass in CI or local validation;
- source review confirms no mutation paths;
- output review confirms no secret leakage;
- README, OS roadmap, race plan, x86_64 roadmap, and status tracker all reflect the implemented B1 boundary;
- non-claims remain intact.

## Related docs

- [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md)
- [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md)
- [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md)
- [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md)
- [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md)

## Non-claims

This validation report does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It records the validation expectations for the first B1 read-only detection script.
