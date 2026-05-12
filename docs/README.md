# Phase1 Documentation

Welcome to the repository-first documentation home for Phase1.

The long-form manual is organized as **The Phase1 Codex: Building a Terminal-First Operating World**.

Start with [`MANUAL_ROADMAP.md`](MANUAL_ROADMAP.md) for the full book, manual, and wiki plan. Use [`REPOSITORY_NAVIGATION.md`](REPOSITORY_NAVIGATION.md) when you need the fastest path to a repository area, support template, contribution workflow, or validation gate.

## Status boundary

> **Status:** Documentation architecture and manual roadmap.
>
> **Validation:** Markdown files and docs guard tests in this repository.
>
> **Non-claims:** Phase1 is not currently a finished secure OS replacement, Base1 is not currently a released bootable daily-driver image, and Fyr is not currently claimed as a production language.

## Fast navigation

- [`REPOSITORY_NAVIGATION.md`](REPOSITORY_NAVIGATION.md) — repository map, fast paths, reader paths, issue-template chooser, quality-gate chooser, and reorganization rules.
- [`REORGANIZATION_PLAN.md`](REORGANIZATION_PLAN.md) — minimalist target structure, destination map, move policy, and rollback rules.
- [`../README.md`](../README.md) — public project entry point and quick start.
- [`../CONTRIBUTING.md`](../CONTRIBUTING.md) — repository-wide contribution guidelines.
- [`../docs/quality/QUALITY.md`](../docs/quality/QUALITY.md) — quality gates and validation commands.
- [`../SECURITY.md`](../SECURITY.md) — security model and reporting guidance.

## Reader paths

| Reader | Start | Purpose |
| --- | --- | --- |
| First-time user | [`operators/README.md`](operators/README.md) | Launch Phase1 safely and understand the current boundary. |
| Operator | [`phase1/README.md`](phase1/README.md) | Learn boot modes, shell workflows, VFS, host tools, and logs. |
| Repository navigator | [`REPOSITORY_NAVIGATION.md`](REPOSITORY_NAVIGATION.md) | Find the right repo area, support path, contribution path, or validation gate quickly. |
| Repository organizer | [`REORGANIZATION_PLAN.md`](REORGANIZATION_PLAN.md) | Follow preservation-first moves, destination mapping, compatibility, and rollback rules. |
| Contributor | [`../CONTRIBUTING.md`](../CONTRIBUTING.md) | Prepare repository contributions while preserving quality, safety, compatibility, and non-claims. |
| Developer | [`developers/README.md`](developers/README.md) | Extend Phase1 while preserving tests, docs, and capability metadata. |
| Security reviewer | [`security/TRUST_MODEL.md`](security/TRUST_MODEL.md) | Review safety claims, trust boundaries, and host-tool behavior. |
| Recovery/hardware operator | [`recovery/README.md`](recovery/README.md) | Follow Base1 and recovery planning without destructive assumptions. |
| Fyr contributor | [`fyr/README.md`](fyr/README.md) | Work on the Phase1-native language and toolchain track. |
| Community/support contributor | [`community/README.md`](community/README.md) | Plan support, forum, and community workflows without asking users to expose private data. |
| Website/branding contributor | [`website/README.md`](website/README.md) | Work on public website structure, branding, content accuracy, accessibility, and mobile fit. |
| Release docs organizer | [`releases/README.md`](releases/README.md) | Organize release notes and checkpoint docs while preserving compatibility paths. |
| Examples/tools organizer | [`../examples/README.md`](../examples/README.md) | Place safe examples and future internal tooling in the right locations. |
| Documentation contributor | [`templates/README.md`](templates/README.md) | Start from reusable status blocks, page skeletons, and claim-review examples. |

## Manual sections

- [`REPOSITORY_NAVIGATION.md`](REPOSITORY_NAVIGATION.md) — repository navigation guide and fast-path map.
- [`REORGANIZATION_PLAN.md`](REORGANIZATION_PLAN.md) — repository reorganization plan and minimalist target structure.
- [`phase1/`](phase1/) — Phase1 Operator Manual.
- [`base1/`](base1/) — Base1 Recovery and OS Foundation Manual.
- [`fyr/`](fyr/) — Fyr Language Book.
- [`operators/`](operators/) — Operator workflows.
- [`developers/`](developers/) — Developer contribution guide.
- [`community/`](community/) — Community support and forum planning.
- [`recovery/`](recovery/) — Recovery and hardware planning.
- [`security/`](security/) — Trust model, claims policy, and review guide.
- [`releases/`](releases/) — Organized release and checkpoint documentation.
- [`website/`](website/) — Website, branding, accessibility, and public content planning.
- [`templates/`](templates/) — Reusable Codex status blocks, page skeletons, and claim-review examples.

## Repository destination indexes

- [`../examples/README.md`](../examples/README.md) — safe examples, walkthrough inputs, Fyr scripts, and dry-run demo material.
- [`../tools/README.md`](../tools/README.md) — internal maintainer utilities, local helpers, future automation, and repo-maintenance tools.
- [`releases/README.md`](releases/README.md) — release documentation destination index.
- [`website/README.md`](website/README.md) — website documentation destination index.

## Contribution entry points

- [`../CONTRIBUTING.md`](../CONTRIBUTING.md) — repository-wide contribution guidelines.
- [`developers/README.md`](developers/README.md) — developer reader path.
- [`developers/DOCS_CONTRIBUTING.md`](developers/DOCS_CONTRIBUTING.md) — documentation contribution guide.
- [`developers/PR_CHECKLIST.md`](developers/PR_CHECKLIST.md) — documentation PR checklist.
- [`../.github/pull_request_template.md`](../.github/pull_request_template.md) — pull request template.

## Support and issue entry points

- [`../.github/ISSUE_TEMPLATE/bug_report.yml`](../.github/ISSUE_TEMPLATE/bug_report.yml) — reproducible defects.
- [`../.github/ISSUE_TEMPLATE/support_request.yml`](../.github/ISSUE_TEMPLATE/support_request.yml) — help using the project.
- [`../.github/ISSUE_TEMPLATE/feature_request.yml`](../.github/ISSUE_TEMPLATE/feature_request.yml) — proposed improvements.
- [`../.github/ISSUE_TEMPLATE/documentation_issue.yml`](../.github/ISSUE_TEMPLATE/documentation_issue.yml) — documentation problems.
- [`community/README.md`](community/README.md) — community support planning.

## Required page status block

New manual pages should include a status block near the top:

```md
> **Status:** Implemented | Experimental | Design | Dry-run | Preview | Roadmap | Not claimed
> **Validation:** tests, scripts, release notes, or manual verification path
> **Non-claims:** what this page does not guarantee
```

## Canonical safety language

Use narrow, testable statements. Do not claim that Phase1 is secure, hardened, bootable, daily-driver ready, installer-ready, or recovery-complete unless the claim is backed by implementation, tests, release notes, and validation evidence.

## Archive

- [`archive/README.md`](archive/README.md) — preserved older root notes, historical checkpoints, and legacy update notes moved out of the repository root.
