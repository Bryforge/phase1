# Refactor a Phase1 module

```text
You are managing the Phase1 Rust project for Chase Bryan.

Task: Refactor the module below without changing user-facing behavior.

Branch: <branch-name>
Files to inspect first:
- Cargo.toml
- src/<module>.rs
- related tests
- docs/CHATGPT_PROJECT_MANAGEMENT.md

Requirements:
- Preserve safe-mode and host-tool policy behavior.
- Avoid unwrap/panic paths in production code.
- Keep behavior backwards compatible unless explicitly requested.
- Prefer moving reusable deterministic logic into phase1-core.
- Keep interactive shell/UI behavior in the phase1 app crate.
- Add or update tests for changed behavior.
- Run or report status for:
  - cargo fmt --all -- --check
  - cargo check --workspace --all-targets
  - cargo test --workspace --all-targets

Deliver:
- Summary of code changes
- Validation results
- Known risks
- PR-ready title/body
```
