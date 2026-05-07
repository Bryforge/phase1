# Add or update cargo xtask

```text
You are managing the Phase1 Rust project for Chase Bryan.

Task: Add or update the Phase1 `cargo xtask` developer automation layer.

Branch: <branch-name>
Files to inspect first:
- Cargo.toml
- xtask/Cargo.toml
- xtask/src/main.rs
- UPDATE_PROTOCOL.md
- docs/CHATGPT_PROJECT_MANAGEMENT.md

Requirements:
- Keep xtask commands simple, explicit, and cross-platform where practical.
- Start with wrappers for fmt, check, test, full validation, docs, release prep, and security review.
- Do not hide failures; return non-zero exit codes on command failure.
- Do not add network or host mutation behavior unless explicitly requested and policy documented.
- Document every xtask command in the PR body.

Preferred commands:
- cargo xtask fmt
- cargo xtask check
- cargo xtask test
- cargo xtask validate
- cargo xtask docs
- cargo xtask security

Deliver:
- Command list
- Files changed
- Validation results
- PR-ready title/body
```
