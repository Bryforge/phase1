# Phase1 Documentation

Welcome to the repository-first documentation home for Phase1.

The long-form manual is organized as **The Phase1 Codex: Building a Terminal-First Operating World**.

Start with [`MANUAL_ROADMAP.md`](MANUAL_ROADMAP.md) for the full book, manual, and wiki plan.

## Status boundary

> **Status:** Documentation architecture and manual roadmap.
>
> **Validation:** Markdown files and docs guard tests in this repository.
>
> **Non-claims:** Phase1 is not currently a finished secure OS replacement, Base1 is not currently a released bootable daily-driver image, and Fyr is not currently claimed as a production language.

## Reader paths

| Reader | Start | Purpose |
| --- | --- | --- |
| First-time user | [`operators/README.md`](operators/README.md) | Launch Phase1 safely and understand the current boundary. |
| Operator | [`phase1/README.md`](phase1/README.md) | Learn boot modes, shell workflows, VFS, host tools, and logs. |
| Contributor | [`../CONTRIBUTING.md`](../CONTRIBUTING.md) | Prepare repository contributions while preserving quality, safety, compatibility, and non-claims. |
| Developer | [`developers/README.md`](developers/README.md) | Extend Phase1 while preserving tests, docs, and capability metadata. |
| Security reviewer | [`security/TRUST_MODEL.md`](security/TRUST_MODEL.md) | Review safety claims, trust boundaries, and host-tool behavior. |
| Recovery/hardware operator | [`recovery/README.md`](recovery/README.md) | Follow Base1 and recovery planning without destructive assumptions. |
| Fyr contributor | [`fyr/README.md`](fyr/README.md) | Work on the Phase1-native language and toolchain track. |
| Community/support contributor | [`community/README.md`](community/README.md) | Plan support, forum, and community workflows without asking users to expose private data. |
| Documentation contributor | [`templates/README.md`](templates/README.md) | Start from reusable status blocks, page skeletons, and claim-review examples. |

## Manual sections

- [`phase1/`](phase1/) — Phase1 Operator Manual.
- [`base1/`](base1/) — Base1 Recovery and OS Foundation Manual.
- [`fyr/`](fyr/) — Fyr Language Book.
- [`operators/`](operators/) — Operator workflows.
- [`developers/`](developers/) — Developer contribution guide.
- [`community/`](community/) — Community support and forum planning.
- [`recovery/`](recovery/) — Recovery and hardware planning.
- [`security/`](security/) — Trust model, claims policy, and review guide.
- [`templates/`](templates/) — Reusable Codex status blocks, page skeletons, and claim-review examples.

## Contribution entry points

- [`../CONTRIBUTING.md`](../CONTRIBUTING.md) — repository-wide contribution guidelines.
- [`developers/README.md`](developers/README.md) — developer reader path.
- [`developers/DOCS_CONTRIBUTING.md`](developers/DOCS_CONTRIBUTING.md) — documentation contribution guide.
- [`developers/PR_CHECKLIST.md`](developers/PR_CHECKLIST.md) — documentation PR checklist.
- [`../.github/pull_request_template.md`](../.github/pull_request_template.md) — pull request template.

## Required page status block

New manual pages should include a status block near the top:

```md
> **Status:** Implemented | Experimental | Design | Dry-run | Preview | Roadmap | Not claimed
> **Validation:** tests, scripts, release notes, or manual verification path
> **Non-claims:** what this page does not guarantee
```

## Canonical safety language

Use narrow, testable statements. Do not claim that Phase1 is secure, hardened, bootable, daily-driver ready, installer-ready, or recovery-complete unless the claim is backed by implementation, tests, release notes, and validation evidence.
