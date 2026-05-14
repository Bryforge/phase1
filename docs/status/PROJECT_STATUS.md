# Public project status

Status kind: estimated roadmap progress
Source marker: [`site/status.json`](../../site/status.json)
Badge marker: [`site/status-badge.json`](../../site/status-badge.json)
Generated from commit: `a495bed2b25a722302a0f47414d4441d59f535b5`
Last updated UTC: `2026-05-14T01:22:43Z`

## Current estimate

| Project | Estimated completion | Current state | Next milestone |
| --- | ---: | --- | --- |
| Phase1 operator console | 82% | usable edge console with guarded host access, VFS, dashboards, help UI, themes, learning, and tests | polish release-facing flows and keep safe defaults simple |
| Fyr native language | 44% | seed language and toolchain surface exist with scripts, tests, assertions, package checks, and docs | expand language book, package workflow, and runtime integration |
| Base1 secure host / OS track | 40% | B2 focused test-suite evidence passed, reviewed B3 VM evidence is present, and the B6 X200 marker chain is published through evidence, checkpoint, public status, and release note; claim remains not_claimed | continue B4 recovery validation and repeatable physical boot evidence while preserving installer, hardening, release-candidate, and daily-driver non-claims |
| X200 / Libreboot hardware path | 44% | X200 Linux-libre host generated reviewed B3 VM evidence and B6 marker evidence with phase1_marker_seen; the checkpoint and release note are published; repeatable physical boot validation remains separate | capture repeatable physical boot evidence and keep emulator, USB, recovery, installer, and hardware-readiness claims separated |
| Security and crypto policy | 55% | trust model, crypto policy roadmap, provider registry, profile docs, config schema, and integrity checks are present | move from documentation policy into scoped implementation only after tests and review evidence |
| Website and public docs | 88% | public site, status page, status JSON, badge endpoint, native GitHub Wiki, refreshed source wiki, organized docs, X200 evidence report, B6 checkpoint trail, and Base1 B6 X200 release note are in place | publish the public report announcement and keep claims evidence-bound |
| Repository organization | 100% | minimal root has 12 tracked files, 16 top-level folders, 0 unplanned root files, 0 tracked build files, and 0 root status duplicates | keep generated artifacts out of Git and keep compatibility links clean as work lands |

Overall estimated roadmap completion: **65%**.

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

## Current public report

Current report: [`docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md`](../../docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md)

The B6 X200 marker chain is now published through raw evidence, checkpoint, public status, and Base1 checkpoint release note.

| Item | Value |
| --- | --- |
| Marker result | `phase1_marker_seen` |
| Claim state | `not_claimed` |
| Checkpoint | [`docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md`](../../docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md) |
| Release note | [`docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md`](../../docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md) |
| Final evidence anchor | `095786e808d3908d27c045f04f3de0b5cd538ab9` |
| Artifact SHA256 | `688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b` |

This report does not claim installer readiness, recovery completion, hardening, release-candidate readiness, daily-driver readiness, or broad hardware validation.

## Non-claims

These percentages are planning estimates. They do not claim that Phase1, Base1, or Fyr are production-ready, installer-ready, daily-driver ready, hardware-validated across targets, hardened, or cryptographically complete.

B6 X200 marker evidence is a named marker observation only; it does not claim installer readiness, recovery completion, hardening, release-candidate readiness, daily-driver readiness, or broad hardware validation.
