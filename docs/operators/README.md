# Operator Reader Path

> **Status:** Manual index and operator workflow map.
>
> **Validation:** Links to current docs and future operator chapters.
>
> **Non-claims:** Operator workflows do not make host-backed actions safe by themselves and do not replace host OS hardening.

This page is the operator entry point for **The Phase1 Codex**.

## First run path

1. Read [`../README.md`](../README.md) to understand the documentation status boundary.
2. Read [`../phase1/README.md`](../phase1/README.md) to understand Phase1 as an operator console.
3. Read [`../security/TRUST_MODEL.md`](../security/TRUST_MODEL.md) before enabling host-backed workflows.
4. Launch Phase1 with the documented quick-start path from the repository README.
5. Inspect the safe shield, trust state, help UI, command registry, and version output.
6. Use read-only or dry-run workflows before host mutation.

## Daily operator path

Use this path for routine operation:

- Boot selector and safe mode.
- Help UI and man-style command pages.
- VFS and simulated system inspection.
- Ops logs and local artifacts.
- Guarded host tools only when needed.
- Troubleshooting through doctor, selftest, version, and status commands.

## Before using host tools

Before enabling or running host-backed behavior, verify:

- the required host capability is named;
- the command explains what it may read or write;
- a dry-run or read-only path exists when possible;
- the trust gate is visible;
- explicit operator confirmation is required for mutation;
- the action is logged for review.

## Operator rule

Phase1 can make actions visible, gated, documented, and logged. It does not make an untrusted host trustworthy and does not replace a VM, container, hardened OS, or external security review.
