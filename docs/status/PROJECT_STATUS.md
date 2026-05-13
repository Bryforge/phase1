# Public project status

Status kind: estimated roadmap progress
Source marker: [`site/status.json`](../../site/status.json)
Badge marker: [`site/status-badge.json`](../../site/status-badge.json)
Generated from commit: `552b27db3fa14a3344cd8d90f7221afda3fb6a78`
Last updated UTC: `2026-05-13T00:28:31Z`

## Current estimate

| Project | Estimated completion | Current state | Next milestone |
| --- | ---: | --- | --- |
| Phase1 operator console | 82% | usable edge console with guarded host access, VFS, dashboards, help UI, themes, learning, and tests | polish release-facing flows and keep safe defaults simple |
| Fyr native language | 44% | seed language and toolchain surface exist with scripts, tests, assertions, package checks, and docs | expand language book, package workflow, and runtime integration |
| Base1 secure host / OS track | 31% | documentation, dry-run scripts, release archives, validation gates, recovery planning, and x86_64 boot planning exist | advance from dry-run/read-only evidence to VM and hardware validation |
| X200 / Libreboot hardware path | 38% | USB staging, framebuffer proof paths, recovery notes, and safety gates exist; hardware success remains evidence-bound | capture repeatable physical boot evidence without strengthening claims early |
| Security and crypto policy | 55% | trust model, crypto policy roadmap, provider registry, profile docs, config schema, and integrity checks are present | move from documentation policy into scoped implementation only after tests and review evidence |
| Website and public docs | 78% | public site, status page, status JSON, badge endpoint, organized docs, asset policy, and Pages routing are in place | keep the Pages workflow generating public status from repository state |
| Repository organization | 100% | minimal root has 12 tracked files, 16 top-level folders, 0 unplanned root files, 0 tracked build files, and 0 root status duplicates | keep generated artifacts out of Git and keep compatibility links clean as work lands |

Overall estimated roadmap completion: **61%**.

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

These percentages are planning estimates. They do not claim that Phase1, Base1, or Fyr are production-ready, installer-ready, daily-driver ready, hardware-validated across targets, hardened, or cryptographically complete.
