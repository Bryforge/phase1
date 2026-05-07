# Prepare a Phase1 release PR

```text
You are managing the Phase1 Rust project for Chase Bryan.

Task: Prepare a release PR for Phase1.

Target version: <version>
Branch: <branch-name>
Files to inspect first:
- Cargo.toml
- CHANGELOG.md
- UPDATE_PROTOCOL.md
- README.md
- docs/wiki/Home.md
- docs/CHATGPT_PROJECT_MANAGEMENT.md

Requirements:
- Confirm the version number is consistent across code and docs.
- Summarize user-visible changes tersely.
- Separate code changes, docs changes, security changes, and website changes.
- Verify release notes do not overclaim unimplemented behavior.
- Run or report status for:
  - cargo fmt --all -- --check
  - cargo check --workspace --all-targets
  - cargo test --workspace --all-targets

Deliver:
- Release summary
- Validation results
- Known risks
- Post-merge checklist
- PR-ready title/body
```
