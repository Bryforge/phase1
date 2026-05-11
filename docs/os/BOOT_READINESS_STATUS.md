# Base1 boot readiness status

Status: active readiness tracker
Scope: Base1 boot readiness levels, open prerequisites, validation evidence, and coding-start gate

## Purpose

This tracker shows what must be finished before Phase1/Base1 moves from planning back into implementation work for boot readiness.

The goal is to complete the planning, status, checklist, and validation map first, then resume coding with a clear B1 read-only detection target.

## Current readiness level

Current level: **B0 — Documentation ready, nearly complete**

Target next level: **B1 — Read-only detection ready**

Do not claim Base1 boot readiness, installer readiness, hardware validation, hardened status, recovery completion, or daily-driver readiness from this tracker alone.

## Boot readiness ladder

| Level | Name | Status | Required before claim strengthens |
| --- | --- | --- | --- |
| B0 | Documentation ready | Nearly complete | Roadmaps, status tracker, checklist, links, tests. |
| B1 | Read-only detection ready | Planned | Dry-run detection script, no writes, architecture/firmware/boot hints. |
| B2 | Dry-run assembly ready | Not started | Image/install/recovery previews with explicit no-write behavior. |
| B3 | VM boot validated | Not started | VM boot report, logs, known limitations. |
| B4 | Recovery validated | Not started | Emergency shell, recovery media, rollback report. |
| B5 | Physical target validated | Not started | Named hardware validation report. |
| B6 | Release candidate | Not started | Repeatable build, validation bundle, docs, rollback evidence. |

## Finish-before-coding checklist

Before coding the next boot-readiness implementation slice, finish:

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
- [ ] B1 plan tests.
- [ ] B1 plan link from OS roadmap, x86_64 roadmap, and race plan.

## B1 coding-start gate

Coding for B1 can begin when this tracker has:

- link from [`ROADMAP.md`](ROADMAP.md);
- link from [`BOOT_READINESS_RACE_PLAN.md`](BOOT_READINESS_RACE_PLAN.md);
- link from [`X86_64_BOOT_SUPPORT_ROADMAP.md`](X86_64_BOOT_SUPPORT_ROADMAP.md);
- README visibility;
- B1 implementation plan: [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md);
- tests that preserve the status ladder, B1 plan, and non-claims.

## Planned first coding slice

The first implementation slice should be:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
```

Implementation plan:

- [`B1_READ_ONLY_DETECTION_PLAN.md`](B1_READ_ONLY_DETECTION_PLAN.md)

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
| B1 detection script | Not started | `scripts/base1-x86_64-detect.sh` |
| B1 detection tests | Not started | planned |
| VM validation report | Not started | planned |
| Recovery validation report | Not started | planned |
| Hardware validation report | Not started | planned |

## Hardening status

Hardening is a roadmap goal and design direction.

Current status: **planned, evidence-bound**.

Do not describe the current Base1 boot path as hardened until implementation, tests, validation reports, recovery evidence, and review evidence support that claim.

## Non-claims

This status tracker does not make Base1 bootable, installer-ready, recovery-complete, hardened, hardware-validated, release-candidate ready, or daily-driver ready.

It tracks what remains before implementation resumes for the B1 read-only boot detection slice.
