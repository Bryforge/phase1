# Add a Phase1 command

```text
You are managing the Phase1 Rust project for Chase Bryan.

Task: Add a new Phase1 command named `<command>`.

Branch: <branch-name>
Files to inspect first:
- src/registry.rs
- src/commands.rs
- src/main.rs
- src/policy.rs
- docs/wiki/04-Command-Manual.md
- docs/CHATGPT_PROJECT_MANAGEMENT.md

Requirements:
- Add command metadata in src/registry.rs.
- Pick the narrowest capability string.
- If the command touches host tools, it must be gated by policy.
- Add help/man text and examples.
- Add tests for registry lookup, aliases, and command behavior.
- Keep shell-only code out of phase1-core unless reusable logic is needed.
- Run or report status for:
  - cargo fmt --all -- --check
  - cargo check --workspace --all-targets
  - cargo test --workspace --all-targets

Deliver:
- Summary of behavior
- Files changed
- Validation results
- Security/policy notes
- PR-ready title/body
```
