# Base1 boot readiness status

Status: active readiness tracker
Scope: Base1 boot readiness levels, open prerequisites, validation evidence, and coding-start gate

## Purpose

This tracker shows what must be finished before Phase1/Base1 moves from planning back into implementation work for boot readiness.

The finish-first planning layer is complete for B1. The first B1 read-only detection script, guard tests, limitations note, and limitations tests now exist while preserving the B1 boundary.

## Current readiness level

Current level: **B1 — Read-only detection ready, initial script present**

Target next level: **B2 — Dry-run assembly ready**

Do not claim Base1 boot readiness, installer readiness, hardware validation, hardened status, recovery completion, or daily-driver readiness from this tracker alone.

## Boot readiness ladder

| Level | Name | Status | Required before claim strengthens |
| --- | --- | --- | --- |
| B0 | Documentation ready | Complete for B1 start | Roadmaps, status tracker, checklist, links, tests. |
| B1 | Read-only detection ready | Initial script present | Dry-run detection script, no writes, architecture/firmware/boot hints, limitations note. |
| B2 | Dry-run assembly ready | Not started | Image/install/recovery previews with explicit no-write behavior. |
| B3 | VM boot validated | Not started | VM boot report, logs, known limitations. |
| B4 | Recovery validated | Not started | Emergency shell, recovery media, rollback report. |
| B5 | Physical target validated | Not started | Named hardware validation report. |
| B6 | Release candidate | Not started | Repeatable build, validation bundle, docs, rollback evidence. |

## Finish-before-coding checklist

Before coding the B1 implementation slice, the finish-first checklist was completed:

- [x] Boot readiness race plan.
- [x] x86_64 boot support roadmap.
- [x] README boot-readiness and x86_64 references.
- [x] Contribution guidelines for hardening and x86_64 boot work.
- [x] Repository navigation and reorganization indexes.
- [x] Asset index and current Fyr asset references.
- [x] Boot readiness status tracker.
- [x] Boot readiness status tracker tests.
- [x] OS roadmap link to this status tracker.
- [x] README link to this status tracker.
- [x] B1 implementation issue/plan for read-only x86_64 detection.
- [x] B1 plan tests.
- [x] B1 plan link from OS roadmap, x86_64 roadmap, and race plan.

Finish-first status: **complete for B1 implementation start**.

## B1 implementation status

B1 initial implementation is now present:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

Initial B1 tests are present:

```bash
cargo test -p phase1 --test base1_x86_64_detect_script
```

B1 limitations are documented in [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md).

B1 limitations tests are present:

```bash
cargo test -p phase1 --test b1_read_only_detection_limitations_docs
```

The B1 detector must stay inside the read-only, dry-run, non-mutating scope defined by [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md).

## B1 completion checklist

Before B1 is considered complete, confirm:

- [x] B1 implementation plan exists.
- [x] B1 read-only detector script exists.
- [x] B1 detector script tests exist.
- [x] B1 known limitations are documented.
- [x] B1 limitations tests exist.
- [ ] B1 detector test suite passes in CI or local validation.
- [ ] B1 status is linked from README, OS roadmap, race plan, and x86_64 roadmap after implementation.
- [ ] B1 output is reviewed for secret redaction.
- [ ] B1 does not contain mutating boot, disk, package, or network commands.

## Planned first coding slice

The first implementation slice is:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

Implementation plan:

- [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md)

Limitations note:

- [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md)

Expected behavior:

- read-only;
- no host mutation;
- reports `writes: no`;
- reports architecture hints;
- reports firmware hints;
- reports boot-loader hints;
- reports virtualization hints;
- reports storage-layout hints;
- reports recovery availability hints;
- fails closed when required facts are unknown.

## Evidence map

| Evidence | Status | Path |
| --- | --- | --- |
| OS roadmap | Present | [`ROADMAP.md`](ROADMAP.md) |
| Boot readiness race plan | Present | [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md) |
| x86_64 boot support roadmap | Present | [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md) |
| Boot readiness status tracker | Present | [`BOOT_READINESS_STATUS.md`](BOOT_READINESS_STATUS.md) |
| B1 read-only detection plan | Present | [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md) |
| B1 limitations note | Present | [`B1_READ_ONLY_DETECTION_LIMITATIONS.md`](B1_READ_ONLY_DETECTION_LIMITATIONS.md) |
| B1 plan tests | Present | `tests/b1_read_only_detection_plan_docs.rs` |
| B1 limitations tests | Present | `tests/b1_read_only_detection_limitations_docs.rs` |
| B1 detection script | Present | `scripts/base1-x86_64-detect.sh` |
| B1 detection tests | Present | `tests/base1_x86_64_detect_script.rs` |
| VM validation report | Not started | planned |
| Recovery validation report | Not started | planned |
| Hardware validation report | Not started | planned |

## Hardening status

Hardening is a roadmap goal and design direction.

Current status: **planned, evidence-bound**.

Do not describe the current Base1 boot path as hardened until implementation, tests, validation reports, recovery evidence, and review evidence support that claim.

## Non-claims

This status tracker does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It records that the first B1 read-only detection script exists and remains bounded to detection-preview behavior only.
