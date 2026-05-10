# Developer Reader Path

> **Status:** Manual index and contribution map.
>
> **Validation:** Links to repository docs, tests, and documentation policy.
>
> **Non-claims:** Developer docs do not make experimental APIs stable and do not treat roadmap work as implemented.

This page is the developer entry point for **The Phase1 Codex**.

## Development reading order

1. Read [`../README.md`](../README.md) for the documentation status model.
2. Read [`../MANUAL_ROADMAP.md`](../MANUAL_ROADMAP.md) for the full Codex structure.
3. Read [`DOCS_CONTRIBUTING.md`](DOCS_CONTRIBUTING.md) before preparing documentation PRs.
4. Read [`PR_CHECKLIST.md`](PR_CHECKLIST.md) before opening documentation PRs.
5. Read [`../security/DOCS_CLAIMS.md`](../security/DOCS_CLAIMS.md) before editing safety, Base1, recovery, installer, or host-tool docs.
6. Read [`../security/TRUST_MODEL.md`](../security/TRUST_MODEL.md) before adding or documenting host-backed behavior.
7. Use the existing repository tests as the source of truth for implemented behavior.

## Required contribution habits

When adding or changing Phase1 features, also update:

- command help or man-style command pages;
- capability or trust-gate documentation for host-backed commands;
- operator-visible examples;
- tests for implemented behavior;
- documentation status blocks when behavior changes;
- the PR checklist when review expectations change.

## Safety-sensitive changes

Treat these as safety-sensitive:

- host tool execution;
- filesystem mutation outside the Phase1 model;
- Git, Cargo, network, or shell workflows;
- Base1 image, boot, recovery, rollback, or installer behavior;
- target identity verification;
- secret handling;
- claims that use words such as secure, safe, hardened, bootable, installer-ready, recovery-complete, or daily-driver ready.

Review safety-sensitive documentation with [`../security/REVIEW_GUIDE.md`](../security/REVIEW_GUIDE.md).

## Developer rule

Every implementation claim should have a runnable path, a test path, or a linked validation artifact. If the proof does not exist, document the item as design, dry-run, preview, roadmap, or not claimed.
