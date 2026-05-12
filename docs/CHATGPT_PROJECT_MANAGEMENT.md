# ChatGPT Project Management for Phase1

Phase1 is now managed with a ChatGPT-first workflow. The goal is to keep every change reviewable, testable, and easy for AI tools to reason about without sacrificing Rust quality or operator safety.

## Operating model

1. Work from a focused feature branch.
2. Keep changes small enough to review.
3. State the goal, touched files, validation commands, and known risks in every PR.
4. Prefer core-library changes in `phase1-core` when logic must be reused by Base1, tests, automation, or AI tooling.
5. Keep interactive terminal behavior in the `phase1` application crate.
6. Do not add host-tool execution paths without policy gating and documentation.

## ChatGPT management loop

Use this loop for every project task:

```text
Intake -> Inspect -> Plan -> Patch -> Validate -> Review -> PR -> Merge notes
```

### Intake

Capture the requested outcome, branch name, safety constraints, and whether the change is code, docs, release, security, UI, Base1, or website work.

### Inspect

Read the relevant files before editing. For Rust changes, inspect `Cargo.toml`, the target module, related tests, and command registry metadata.

### Plan

Produce a short implementation plan before making multi-file changes. Include any assumptions and fallback choices.

### Patch

Make the smallest coherent change. Avoid mixing unrelated UI, security, docs, and architecture work in the same PR unless the task explicitly requires it.

### Validate

Preferred local validation:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
```

For security or host-tool changes, also inspect `src/policy.rs`, `SECURITY.md`, and `docs/security/SECURITY_REVIEW.md`.

### Review

Summarize changed behavior, affected commands, risks, and test coverage. If validation could not be run, state that clearly.

### PR

Open a PR with:

- summary
- validation
- known risks
- follow-up recommendations

### Merge notes

After merge, update any release notes, roadmap, or user-facing docs affected by the change.

## Core extraction rule

`phase1-core` should contain deterministic reusable logic. It should avoid terminal input loops, direct host command execution, and app-shell-only state. If a source file depends on `Phase1Shell`, split the shell-free logic into `phase1-core` first, then adapt the shell layer separately.

## AI notes convention

Use `AI-NOTE:` comments sparingly for architectural guardrails that future AI assistants must preserve. Do not use them for obvious Rust syntax or temporary TODOs.

## Safety baseline

ChatGPT-managed changes must preserve:

- safe mode default behavior
- explicit host-tool opt-in
- credential redaction in history and ops logs
- VFS-only behavior for editors unless explicitly documented otherwise
- clear distinction between simulated system behavior and real host behavior
