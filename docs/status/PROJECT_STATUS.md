# Public project status

Status kind: estimated roadmap progress
Source marker: [`site/status.json`](../../site/status.json)
Badge marker: [`site/status-badge.json`](../../site/status-badge.json)
Generated from commit: `910c60d227966c0fdda8c605e2ba8b092defcdb7`
Last updated UTC: `2026-05-14T20:41:34Z`

## Current estimate

| Project | Estimated completion | Current state | Next milestone |
| --- | ---: | --- | --- |
| Phase1 operator console | 82% | usable edge console with guarded host access, VFS, dashboards, help UI, themes, learning, and tests | wire safe Fyr black_arts staged runtime stubs while preserving non-live defaults |
| Fyr native language | 58% | seed language and toolchain surface exist with F3/F4/F5/F6 evidence, runtime-safety fixtures, standard-library contracts, Fyr-aware tab completion, and black_arts staged-candidate design/operator-visual/source-wiring handoff evidence | implement the first safe fyr staged runtime stub from issue #317 without candidate writes, host commands, network access, validation execution, promotion, discard, or live-system changes |
| Base1 secure host / OS track | 40% | B2 focused test-suite evidence passed, reviewed B3 VM evidence is present, and the B6 X200 marker chain is published through evidence, checkpoint, public status, and release note; claim remains not_claimed | continue B4 recovery validation and repeatable physical boot evidence while preserving installer, hardening, release-candidate, and daily-driver non-claims |
| X200 / Libreboot hardware path | 44% | X200 Linux-libre host generated reviewed B3 VM evidence and B6 marker evidence with phase1_marker_seen; the checkpoint and release note are published; repeatable physical boot validation remains separate | capture repeatable physical boot evidence and keep emulator, USB, recovery, installer, and hardware-readiness claims separated |
| Security and crypto policy | 55% | trust model, crypto policy roadmap, provider registry, profile docs, config schema, and integrity checks are present | move from documentation policy into scoped implementation only after tests and review evidence |
| Website and public docs | 90% | public site, status page, status JSON, badge endpoint, native GitHub Wiki, refreshed source wiki, organized docs, X200 evidence report, B6 checkpoint trail, Base1 B6 X200 release note, and Fyr black_arts public status trail are in place | keep the public status synchronized with implementation evidence and non-claims |
| Repository organization | 100% | minimal root has 11 tracked files, 17 top-level folders, 0 unplanned root files, 0 tracked build files, and 0 root status duplicates | keep generated artifacts out of Git and keep compatibility links clean as work lands |

Overall estimated roadmap completion: **67%**.

## Repository organization inputs

- Root tracked files: 11
- Top-level tracked directories: 17
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

Current report: Fyr black_arts staged-candidate evidence and [`docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md`](../../docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md)

Fyr now has command and action-aware tab completion for `fyr`, `fyr run`, `fyr cat`, and the default `hello_hacker.fyr` VFS demo; Fyr black_arts also has staged-candidate design, fixture, visual-mode, checklist, and runtime-stub handoff evidence. The first safe runtime wiring remains pending under issue #317.

The B6 X200 marker chain is still published through raw evidence, checkpoint, public status, and Base1 checkpoint release note.

| Item | Value |
| --- | --- |
| Fyr black_arts runtime issue | `#317` |
| Fyr black_arts design | [`docs/fyr/STAGED_CANDIDATES.md`](../../docs/fyr/STAGED_CANDIDATES.md) |
| Fyr black_arts operator visuals | [`docs/fyr/BLACK_ARTS_OPERATOR_VISUALS.md`](../../docs/fyr/BLACK_ARTS_OPERATOR_VISUALS.md) |
| Fyr black_arts wiring handoff | [`docs/fyr/STAGED_RUNTIME_PATCH_HANDOFF.md`](../../docs/fyr/STAGED_RUNTIME_PATCH_HANDOFF.md) |
| Marker result | `phase1_marker_seen` |
| Claim state | `not_claimed` |
| Checkpoint | [`docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md`](../../docs/checkpoints/B6_X200_MARKER_CHECKPOINT.md) |
| Release note | [`docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md`](../../docs/base1/releases/RELEASE_BASE1_B6_X200_MARKER_CHECKPOINT_V1.md) |
| Final evidence anchor | `095786e808d3908d27c045f04f3de0b5cd538ab9` |
| Artifact SHA256 | `688518c1437003c7b8325b1d5d479bc97f77c3404c8fd27dace6d823d406b79b` |

This report does not claim installer readiness, recovery completion, hardening, release-candidate readiness, daily-driver readiness, broad hardware validation, live-system mutation, or autonomous promotion.

## Non-claims

These percentages are planning estimates. They do not claim that Phase1, Base1, or Fyr are production-ready, installer-ready, daily-driver ready, hardware-validated across targets, hardened, cryptographically complete, live-self-updating, or capable of autonomous promotion/mutation.

Fyr black_arts staged-candidate evidence is fixture-backed and design/contract oriented. The first safe runtime wiring remains pending under issue #317.

B6 X200 marker evidence is a named marker observation only; it does not claim installer readiness, recovery completion, hardening, release-candidate readiness, daily-driver readiness, or broad hardware validation.
