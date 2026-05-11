# Phase1 examples

Status: active examples index
Scope: runnable examples, walkthrough inputs, demo scripts, learning samples, and safe operator examples

## Purpose

This directory is the preferred organized home for examples that help users learn Phase1, Base1, and Fyr without mixing examples into source, release notes, or core documentation.

Examples should be easy to copy, safe to run, and honest about what is implemented versus planned.

## Example safety rules

Examples must:

- avoid destructive host commands by default;
- prefer read-only or dry-run behavior;
- avoid secrets, tokens, private keys, credentials, recovery codes, private logs, or unrevised screenshots;
- clearly mark host-backed behavior when relevant;
- avoid claiming production readiness, hardened status, hardware validation, audit status, certification, or quantum safety without evidence;
- link to docs when an example depends on a roadmap or preview feature.

## Planned structure

```text
examples/
  README.md
  phase1/
  fyr/
  base1/
  security/
  community/
```

Create subdirectories only when they contain real examples.

## Suggested categories

| Category | Purpose |
| --- | --- |
| `phase1/` | Shell workflows, VFS examples, safe-mode examples, operator walkthroughs. |
| `fyr/` | Fyr scripts, syntax examples, language walkthroughs, package examples. |
| `base1/` | Read-only Base1 dry-run examples and recovery planning examples. |
| `security/` | Safe redaction, trust-gate, and crypto-policy documentation examples. |
| `community/` | Support templates, issue-draft examples, and forum-support examples. |

## Validation

Before adding or changing examples, run the relevant gate:

```bash
sh scripts/quality-check.sh quick
```

For Fyr examples that execute through the current runtime, add or update tests when behavior is claimed.

For Base1 examples, prefer dry-run commands and run:

```bash
sh scripts/quality-check.sh base1-docs
```

For security or crypto-policy examples, run:

```bash
sh scripts/quality-check.sh security-crypto-docs
```

## Non-claims

This index does not create examples by itself, prove example correctness, launch support infrastructure, or make Phase1, Base1, or Fyr production-ready.

It creates an organized destination for future safe examples.
