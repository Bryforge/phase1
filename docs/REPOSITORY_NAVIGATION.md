# Phase1 repository navigation guide

Status: active navigation guide
Scope: repository organization, reader paths, contribution paths, support paths, and validation paths

## Purpose

This guide helps users, contributors, operators, reviewers, and maintainers find the right part of the Phase1 repository quickly.

Phase1 now covers several connected workstreams:

- Phase1 terminal-first virtual OS console;
- Base1 OS foundation and recovery planning;
- Fyr native language track;
- security and crypto policy planning;
- community support and future support AI planning;
- website, docs, quality, and contribution workflows.

## Fast paths

| Need | Start here |
| --- | --- |
| Run Phase1 | [`../README.md`](../README.md#quick-start) |
| Understand what is implemented | [`../FEATURE_STATUS.md`](../FEATURE_STATUS.md) |
| Learn the docs structure | [`README.md`](README.md) |
| Understand the reorganization plan | [`REORGANIZATION_PLAN.md`](REORGANIZATION_PLAN.md) |
| Contribute | [`../CONTRIBUTING.md`](../CONTRIBUTING.md) |
| Open a PR | [`../.github/pull_request_template.md`](../.github/pull_request_template.md) |
| Report a bug | [`../.github/ISSUE_TEMPLATE/bug_report.yml`](../.github/ISSUE_TEMPLATE/bug_report.yml) |
| Ask for support | [`../.github/ISSUE_TEMPLATE/support_request.yml`](../.github/ISSUE_TEMPLATE/support_request.yml) |
| Request a feature | [`../.github/ISSUE_TEMPLATE/feature_request.yml`](../.github/ISSUE_TEMPLATE/feature_request.yml) |
| Report a docs issue | [`../.github/ISSUE_TEMPLATE/documentation_issue.yml`](../.github/ISSUE_TEMPLATE/documentation_issue.yml) |
| Review security posture | [`security/README.md`](security/README.md) |
| Review crypto policy | [`security/CRYPTO_POLICY_ROADMAP.md`](security/CRYPTO_POLICY_ROADMAP.md) |
| Review quality gates | [`../QUALITY.md`](../QUALITY.md) |
| Review Base1 | [`base1/README.md`](base1/README.md) |
| Review Fyr | [`fyr/README.md`](fyr/README.md) |
| Review community plans | [`community/README.md`](community/README.md) |
| Review release docs organization | [`releases/README.md`](releases/README.md) |
| Review website docs organization | [`website/README.md`](website/README.md) |
| Review examples organization | [`../examples/README.md`](../examples/README.md) |
| Review internal tools organization | [`../tools/README.md`](../tools/README.md) |

## Repository map

| Path | Purpose |
| --- | --- |
| `README.md` | Public project entry point, quick start, status, Base1 overview, contribution link. |
| `CONTRIBUTING.md` | Repository-wide contribution rules and validation expectations. |
| `SECURITY.md` | Security model, trust gates, crypto policy goal, and reporting guidance. |
| `QUALITY.md` | Quality gates, validation commands, score model, and ownership areas. |
| `FEATURE_STATUS.md` | Implemented, experimental, restricted, and roadmap feature matrix. |
| `PHASE1_NATIVE_LANGUAGE.md` | Fyr language specification and entry point. |
| `.github/` | Pull request template, issue templates, workflows, and automation. |
| `src/` | Phase1 Rust source. |
| `src/bin/` | Helper binaries such as storage, install, and learning tools. |
| `phase1-core/` | Core package workspace member. |
| `base1/` | Base1 root-level docs, compatibility paths, hardware/recovery docs. |
| `docs/` | Repository-first manual, navigation, developer docs, security docs, community docs. |
| `docs/REPOSITORY_NAVIGATION.md` | Fast-path map for users, contributors, support, and validation. |
| `docs/REORGANIZATION_PLAN.md` | Minimalist target structure, destination map, move policy, and rollback rules. |
| `docs/releases/` | Organized release docs, checkpoint notes, release mirrors, and release indexes. |
| `docs/website/` | Public website docs, branding notes, content maps, and accessibility planning. |
| `docs/security/` | Trust model, crypto policy, registry, providers, config, implementation plan, review guidance. |
| `docs/community/` | Support forum roadmap and automated support AI roadmap. |
| `docs/base1/` | Organized Base1 manual and release mirrors. |
| `docs/fyr/` | Fyr language manual and roadmap. |
| `examples/` | Safe examples, walkthrough inputs, Fyr scripts, and dry-run demo material. |
| `tools/` | Internal maintainer utilities, local helpers, future automation, and repo-maintenance tools. |
| `scripts/` | Quality gates, Base1 validation, runtime helpers, wiki/docs helpers. |
| `tests/` | Rust tests and docs guard tests. |
| `assets/` | Project imagery, logos, and public-facing visual assets. |

## Reader paths

### First-time user

1. Read [`../README.md`](../README.md).
2. Run the quick start.
3. Read [`operators/README.md`](operators/README.md).
4. Check [`../FEATURE_STATUS.md`](../FEATURE_STATUS.md) before assuming a feature is implemented.

### Contributor

1. Read [`../CONTRIBUTING.md`](../CONTRIBUTING.md).
2. Read [`developers/README.md`](developers/README.md).
3. Use the PR template in [`../.github/pull_request_template.md`](../.github/pull_request_template.md).
4. Run the relevant quality gate before opening work.

### Repository organizer

1. Read [`REORGANIZATION_PLAN.md`](REORGANIZATION_PLAN.md).
2. Create a move map before moving files.
3. Keep compatibility paths unless a tested wrapper, mirror, or index exists.
4. Add or update tests for required navigation links.
5. Run the relevant quality gate before and after reorganization work.

### Release/documentation organizer

1. Read [`releases/README.md`](releases/README.md).
2. Keep root-level release notes and checkpoint files as compatibility paths unless a move map approves a change.
3. Prefer organized mirrors or indexes before moving release files.
4. Run `sh scripts/quality-check.sh quick` and Base1 gates when Base1 release docs are involved.

### Website/asset organizer

1. Read [`website/README.md`](website/README.md).
2. Keep website claims aligned with repository evidence.
3. Keep branding assets under `assets/` unless a future asset map says otherwise.
4. Run website and quality checks before changing public-facing structure.

### Examples/tools organizer

1. Read [`../examples/README.md`](../examples/README.md) for safe example placement.
2. Read [`../tools/README.md`](../tools/README.md) before adding internal helper tooling.
3. Keep user-facing scripts in `scripts/` unless a move map and compatibility wrapper exist.
4. Prefer read-only or dry-run examples.

### Security reviewer

1. Read [`security/README.md`](security/README.md).
2. Read [`../SECURITY.md`](../SECURITY.md).
3. Read [`security/TRUST_MODEL.md`](security/TRUST_MODEL.md).
4. Use [`security/REVIEW_GUIDE.md`](security/REVIEW_GUIDE.md).
5. Preserve non-claims and evidence-backed wording.

### Crypto contributor

1. Read [`security/CRYPTO_POLICY_ROADMAP.md`](security/CRYPTO_POLICY_ROADMAP.md).
2. Read [`security/CRYPTO_IMPLEMENTATION_PLAN.md`](security/CRYPTO_IMPLEMENTATION_PLAN.md).
3. Use [`security/CRYPTO_ALGORITHM_TEMPLATE.md`](security/CRYPTO_ALGORITHM_TEMPLATE.md) for algorithm entries.
4. Use [`security/CRYPTO_PROVIDER_TEMPLATE.md`](security/CRYPTO_PROVIDER_TEMPLATE.md) for provider entries.
5. Run:

```bash
sh scripts/quality-check.sh security-crypto-docs
```

### Base1 contributor

1. Read [`base1/README.md`](base1/README.md).
2. Read [`recovery/README.md`](recovery/README.md).
3. Keep root compatibility paths recoverable.
4. Prefer read-only and dry-run workflows.
5. Run:

```bash
sh scripts/quality-check.sh base1-docs
```

### Fyr contributor

1. Read [`fyr/README.md`](fyr/README.md).
2. Read [`../PHASE1_NATIVE_LANGUAGE.md`](../PHASE1_NATIVE_LANGUAGE.md).
3. Add tests for parser/runtime behavior when changing implemented behavior.
4. Avoid production-ready claims unless evidence exists.

### Community/support contributor

1. Read [`community/README.md`](community/README.md).
2. Read [`community/SUPPORT_FORUM_ROADMAP.md`](community/SUPPORT_FORUM_ROADMAP.md).
3. Read [`community/AUTOMATED_SUPPORT_AI_ROADMAP.md`](community/AUTOMATED_SUPPORT_AI_ROADMAP.md).
4. Keep support workflows secret-safe and security-sensitive reports private.

## Quality gate chooser

| Work type | Recommended gate |
| --- | --- |
| General code/docs work | `sh scripts/quality-check.sh quick` |
| Release-facing work | `sh scripts/quality-check.sh full` |
| Base1 docs work | `sh scripts/quality-check.sh base1-docs` |
| Broad Base1 reorganization | `sh scripts/quality-check.sh base1-reorg` |
| Crypto/security policy docs | `sh scripts/quality-check.sh security-crypto-docs` |
| Script-only syntax check | `sh scripts/quality-check.sh scripts` |
| Required file presence | `sh scripts/quality-check.sh files` |
| Scorecard only | `sh scripts/quality-check.sh score` |

## Issue template chooser

| Situation | Template |
| --- | --- |
| Reproducible defect | Bug report |
| Need help using the project | Support request |
| Proposed improvement | Feature request |
| Missing, confusing, outdated, or unsafe docs | Documentation issue |
| Private vulnerability or sensitive security report | Do not open a public issue; follow `SECURITY.md` |

## Reorganization rules

Repository reorganization should be preservation-first.

Rules:

- Keep existing public/root compatibility paths unless a future plan explicitly replaces them.
- Use [`REORGANIZATION_PLAN.md`](REORGANIZATION_PLAN.md) before broad restructuring.
- Prefer adding indexes and mirrors before moving files.
- Add navigation docs before broad restructuring.
- Add tests for new navigation and required links.
- Keep safety, non-claims, and validation paths visible.
- Keep users one or two clicks away from quick start, contribution rules, issue templates, quality gates, and security reporting.

## Non-claims

This navigation guide does not move files, launch support infrastructure, prove repository quality, or make Phase1, Base1, or Fyr production-ready.

It provides a clearer path through the repository so users and contributors can work more efficiently.
