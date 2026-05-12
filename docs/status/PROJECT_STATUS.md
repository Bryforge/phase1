# Public project status

Status kind: estimated roadmap progress  
Source marker: [`site/status.json`](../../site/status.json)  
Badge marker: [`site/status-badge.json`](../../site/status-badge.json)

## Current estimate

| Project | Estimated completion | Current state | Next milestone |
| --- | ---: | --- | --- |
| Phase1 operator console | 82% | Usable edge console with guarded host access, VFS, dashboards, help UI, themes, learning, and tests. | Polish release-facing flows and keep safe defaults simple. |
| Fyr native language | 44% | Seed language and toolchain surface exist with scripts, tests, assertions, package checks, and docs. | Expand language book, package workflow, and runtime integration. |
| Base1 secure host / OS track | 31% | Documentation, dry-run scripts, release archives, validation gates, recovery planning, and x86_64 boot planning exist. | Advance from dry-run/read-only evidence to VM and hardware validation. |
| X200 / Libreboot hardware path | 38% | USB staging, framebuffer proof paths, recovery notes, and safety gates exist; hardware success remains evidence-bound. | Capture repeatable physical boot evidence without strengthening claims early. |
| Security and crypto policy | 55% | Trust model, crypto policy roadmap, provider registry, profile docs, config schema, and integrity checks are present. | Move from documentation policy into scoped implementation only after tests and review evidence. |
| Website and public docs | 73% | Public site, organized docs, status routing, asset policy, and repository navigation are in place. | Add live project status rendering from the JSON marker. |
| Repository organization | 92% | Minimal root layout and organized docs folders are in place on `edge/stable` and `black-phase1`. | Keep compatibility links and quality gates clean as new work lands. |

Overall estimated roadmap completion: **58%**.

## How to check it publicly

The machine-readable marker lives at:

```text
site/status.json
```

The shield-style endpoint lives at:

```text
site/status-badge.json
```

After GitHub Pages refreshes, public readers can check:

```text
https://bryforge.github.io/phase1/status.json
https://bryforge.github.io/phase1/status-badge.json
```

## Non-claims

These percentages are planning estimates. They do not claim that Phase1, Base1, or Fyr are production-ready, installer-ready, daily-driver ready, hardware-validated across targets, hardened, or cryptographically complete.
