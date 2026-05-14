# Public project status

Status kind: estimated roadmap progress
Source marker: [`site/status.json`](../../site/status.json)
Badge marker: [`site/status-badge.json`](../../site/status-badge.json)
Generated from commit: `d4cd1e13d429662f6713466f57a41233d8238416`
Last updated UTC: `2026-05-14T00:59:29Z`

## Current estimate

| Project | Estimated completion | Current state | Next milestone |
| --- | ---: | --- | --- |
| Phase1 operator console | 82% | usable edge console with guarded host access, VFS, dashboards, help UI, themes, learning, and tests | polish release-facing flows and keep safe defaults simple |
| Fyr native language | 44% | seed language and toolchain surface exist with scripts, tests, assertions, package checks, and docs | expand language book, package workflow, and runtime integration |
| Base1 secure host / OS track | 40% | B2 focused test-suite evidence passed, reviewed B3 VM evidence is present, and the B6 X200 marker checkpoint records `phase1_marker_seen`; claim remains `not_claimed` | continue B4 recovery validation while keeping installer, hardening, release-candidate, and daily-driver claims out of scope |
| X200 / Libreboot hardware path | 44% | X200 Linux-libre host generated reviewed B3 VM evidence and B6 marker checkpoint evidence with `phase1_marker_seen`; repeatable physical boot validation remains separate and not claimed | capture repeatable physical boot evidence and keep emulator, USB, recovery, installer, and hardware-readiness claims separated |
| Security and crypto policy | 55% | trust model, crypto policy roadmap, provider registry, profile docs, config schema, and integrity checks are present | move from documentation policy into scoped implementation only after tests and review evidence |
| Website and public docs | 88% | public site, status page, status JSON, badge endpoint, native GitHub Wiki, refreshed source wiki, organized docs, asset policy, Pages routing, clean public link checks, X200 evidence report, reviewed B3 VM wording, and B6 checkpoint trail are in place | publish B4 recovery validation status and keep public claims evidence-bound |
| Repository organization | 100% | minimal root has 12 tracked files, 16 top-level folders, 0 unplanned root files, 0 tracked build files, 0 root status duplicates, and clean Base1 link-check output | keep generated artifacts out of Git and keep compatibility links clean as work lands |

Overall estimated roadmap completion: **66%**.

## B3/B6 evidence boundary

Reviewed B3 VM evidence is present. This includes B2 focused test-suite evidence, B3 GNU/Linux emulator evidence, B3 UEFI proof evidence, B3 kernel/initrd handoff evidence, OpenBSD launch-stage evidence, and B3 log-bundle review evidence.

The claim state remains `not_claimed`. This is not physical hardware boot validation, installer readiness, recovery validation, hardening proof, release-candidate readiness, or daily-driver readiness.

B6 X200 marker checkpoint is present at [`docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md`](../checkpoints/B6_X200_MARKER_CHECKPOINT.md). It records checkpoint commit `d4cd1e13d429662f6713466f57a41233d8238416`, final evidence anchor `095786e808d3908d27c045f04f3de0b5cd538ab9`, checkpoint source commit `8eeca92294e8fc67437b46f4cb38917a4428e219`, and artifact SHA256 `688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b`.

B6 marker evidence remains a named observation only. It does not claim installer readiness, recovery completion, hardening, release-candidate readiness, daily-driver readiness, or broad hardware validation.

## Repository organization inputs

- Root tracked files: 12
- Top-level tracked directories: 16
- Unplanned root files: 0
- Tracked build files: 0
- Root status duplicates: 0
- Generated artifact count: 0

## How to check it publicly

```text
https://bryforge.github.io/phase1/status.html
https://bryforge.github.io/phase1/status.json
https://bryforge.github.io/phase1/status-badge.json
```

## Non-claims

These percentages are planning estimates. They do not claim that Phase1, Base1, or Fyr are production-ready, installer-ready, daily-driver ready, hardware-validated across targets, hardened, cryptographically complete, or fully hardware-boot validated.

B6 X200 marker evidence is a named marker observation only; it does not claim installer readiness, recovery completion, hardening, release-candidate readiness, daily-driver readiness, or broad hardware validation.
