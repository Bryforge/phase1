# Phase1 Codex PR Checklist

> **Status:** Documentation checklist.
>
> **Validation:** Use during documentation pull requests.
>
> **Non-claims:** This checklist does not validate a feature by itself.

Use this checklist before opening a documentation PR.

## Required checks

- The page has a status block.
- The page names the current behavior or marks the work as planned.
- The page links to related Codex pages when appropriate.
- Examples are current and runnable when they claim current behavior.
- New files are linked from a relevant index.
- Navigation changes include a docs test when practical.
- Wording avoids broad claims without evidence.
- Host-backed, recovery, hardware, or language-tooling pages use the review guide.
- Related docs tests pass locally or in CI.

## Useful commands

```bash
cargo test -p phase1 --test manual_roadmap_docs
cargo test -p phase1 --test codex_templates_docs
```

## Review handoff

In the PR body, include:

- what changed;
- which reader path is affected;
- which docs tests were run;
- whether the change is current behavior, design, preview, or roadmap.
