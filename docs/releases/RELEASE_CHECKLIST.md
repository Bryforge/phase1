# phase1 release checklist

## v3.6.0

### Required local checks

Run from repo root:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo test --test smoke -- --nocapture
cargo build --release
```

### Manual demo check

Inside phase1:

```text
version
dash --compact
capabilities
commands
py -c "print('alias-ok')"
spawn worker --background
ps
jobs
audit
browser phase1
exit
```

### Release artifact checklist

- [x] `Cargo.toml` version bumped to `3.6.0`.
- [x] `src/kernel.rs` runtime version bumped to `3.6.0`.
- [x] `Cargo.lock` refreshed for `3.6.0`.
- [x] README updated.
- [x] CHANGELOG added.
- [x] v3.6.0 release notes added.
- [x] Smoke tests updated for v3.6.0.
- [ ] Local Rust checks completed on a machine with Rust installed.
- [ ] GitHub release object created manually if desired.

## Tagging guidance

A release branch/ref can point at the v3.6.0 commit. A GitHub Release can then be created from that ref with the contents of `docs/releases/RELEASE_NOTES_v3.6.0.md`.
