# Phase1 Codex Documentation Contribution Guide

> **Status:** Documentation governance guide.
>
> **Validation:** Use with `docs/security/DOCS_CLAIMS.md`, `docs/security/TRUST_MODEL.md`, and repository docs tests.
>
> **Non-claims:** This guide does not certify a feature, release, recovery workflow, or security property. It defines how documentation changes should be prepared and reviewed.

This guide turns **The Phase1 Codex** rules into a practical checklist for contributors.

## Required page shape

Every major Codex page should include:

```md
> **Status:** Implemented | Experimental | Design | Dry-run | Preview | Roadmap | Not claimed
> **Validation:** tests, scripts, release notes, or manual verification path
> **Non-claims:** what this page does not guarantee
```

Use the strongest status only when the repository evidence supports it.

## Contributor workflow

1. Identify whether the change documents current behavior, experimental behavior, design, dry-run work, preview work, roadmap work, or an explicit non-claim.
2. Link to the source of truth instead of duplicating long sections from another page.
3. Add runnable examples only when they match current behavior.
4. Put dry-run or read-only examples before mutation examples.
5. Add or update docs tests when changing important navigation, claims, or safety wording.
6. Keep examples free of secrets, real tokens, personal data, and host-specific private paths.

## Safety-sensitive documentation

Treat these topics as safety-sensitive:

- host-backed tools;
- filesystem mutation outside the Phase1 model;
- Git, Cargo, shell, or network workflows;
- Base1 boot, image, installer, rollback, or recovery material;
- hardware validation;
- target identity checks;
- security claims;
- secret handling.

Safety-sensitive pages must be reviewed against [`../security/REVIEW_GUIDE.md`](../security/REVIEW_GUIDE.md).

## Allowed claim pattern

Prefer this structure:

```md
Current behavior: what exists now.
Validation: how to verify it.
Limitations: what it does not prove.
Roadmap: what may come later.
```

## Disallowed shortcuts

Do not make broad claims such as secure, safe, hardened, production-ready, installer-ready, recovery-complete, daily-driver ready, or bootable without linked evidence and review.

## PR checklist

Before opening a docs PR:

- The page has a status block.
- The page separates current behavior from roadmap work.
- Every safety claim names the mechanism.
- Every strong claim has evidence.
- Destructive examples are not the first example.
- Host-backed behavior names the host boundary.
- Base1 and recovery language stays validation-gated.
- Fyr language claims stay tied to current examples, tests, or roadmap labels.
- Links to Codex index pages still work.
- Relevant docs tests pass.
