# Public project status

Status kind: estimated roadmap progress
Source marker: [`site/status.json`](../../site/status.json)
Badge marker: [`site/status-badge.json`](../../site/status-badge.json)
Generated from commit: `74c3c97e2ff2f6e95d3aa4ea1156eeaf83953b41`
Last updated UTC: `2026-05-13T21:42:00Z`

## Current estimate

| Project | Estimated completion | Current state | Next milestone |
| --- | ---: | --- | --- |
| Phase1 operator console | 82% | usable edge console with guarded host access, VFS, dashboards, help UI, themes, learning, and tests | polish release-facing flows and keep safe defaults simple |
| Fyr native language | 44% | seed language and toolchain surface exist with scripts, tests, assertions, package checks, and docs | expand language book, package workflow, and runtime integration |
| Base1 secure host / OS track | 38% | documentation, dry-run scripts, release archives, validation gates, recovery planning, x86_64 boot planning, wiki guardrails, B3 GNU/Linux emulator evidence, B3 UEFI emulator proof, and B3 kernel/initrd handoff evidence are present; full B3 remains not claimed | promote a reviewed B3 validation report without over-claiming, then move toward recovery and physical hardware evidence |
| X200 / Libreboot hardware path | 42% | X200 Linux-libre host can stage local kernel/initrd into B3 GNU/Linux emulator evidence, run B3 UEFI proof, and produce B3 kernel/initrd handoff evidence; USB staging, framebuffer proof paths, recovery notes, and safety gates remain evidence-bound | capture repeatable physical boot evidence and keep emulator, USB, and hardware claims separated |
| Security and crypto policy | 55% | trust model, crypto policy roadmap, provider registry, profile docs, config schema, and integrity checks are present | move from documentation policy into scoped implementation only after tests and review evidence |
| Website and public docs | 87% | public site, status page, status JSON, badge endpoint, native GitHub Wiki, refreshed source wiki, organized docs, asset policy, Pages routing, clean public link checks, and B3 emulator evidence-set status wording are in place | publish the reviewed B3 validation report and keep public claims evidence-bound |
| Repository organization | 100% | minimal root has 12 tracked files, 16 top-level folders, 0 unplanned root files, 0 tracked build files, 0 root status duplicates, and clean Base1 link-check output | keep generated artifacts out of Git and keep compatibility links clean as work lands |

Overall estimated roadmap completion: **65%**.

## B3 evidence boundary

B3 GNU/Linux emulator stage evidence, B3 UEFI proof evidence, and B3 kernel/initrd handoff evidence are present. Full B3 validation remains incomplete and not claimed until a reviewed validation report exists.

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

These percentages are planning estimates. They do not claim that Phase1, Base1, or Fyr are production-ready, installer-ready, daily-driver ready, hardware-validated across targets, hardened, cryptographically complete, or fully B3-complete.
