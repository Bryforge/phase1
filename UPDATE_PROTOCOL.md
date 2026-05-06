# phase1 update protocol

This file is the canonical update reference for phase1. Use it before changing update behavior, release metadata, README release notes, or roadmap status.

## Version format

phase1 uses semantic versioning everywhere:

```text
MAJOR.MINOR.PATCH[-dev]
```

Examples:

```text
3.6.0       stable release
3.7.0-dev   first bleeding-edge build after v3.6.0
3.7.1-dev   next bleeding-edge patch or incremental feature update
```

## Version bump rules

- **PATCH**, the third number, is required for every safe fix, documentation update, update-protocol improvement, formatting-only update, and incremental bleeding-edge feature.
- **MINOR**, the second number, is used when a roadmap track gains a meaningful new capability set.
- **MAJOR**, the first number, is reserved for deliberate compatibility-breaking behavior.
- Bleeding-edge builds keep the `-dev` suffix until they are promoted into a release.
- Stable release tags must use exact versions without `-dev`.

## Current channels

```text
release        3.6.0
bleeding edge  3.7.1-dev
branch         master
stable branch  stable
```

## Safe update command behavior

The in-app updater must remain safe by default.

```text
update                         print the dry-run plan only
update bleeding --check         inspect local Git state only
update bleeding --execute       run guarded fetch/checkout/pull
update bleeding --execute --build
```

Execution rules:

- `update` without `--execute` must never modify files.
- `update --execute` must require `PHASE1_SAFE_MODE=0` and `PHASE1_ALLOW_HOST_TOOLS=1`.
- Host network mutation remains separately gated by `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1`.
- Tracked local changes must block update execution instead of being overwritten.
- Updater output must redact GitHub tokens, URL credentials, passwords, cookies, private keys, and account-like secrets.
- The updater must not ask for GitHub passwords, email passwords, Apple ID credentials, personal access tokens, recovery codes, SSH private keys, or browser cookies.

## Required checks for update changes

Run these before publishing update-related changes:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo test --test smoke -- --nocapture
cargo test --test bleeding -- --nocapture
```

## Files to update together

When version metadata or update behavior changes, update these files together:

```text
UPDATE_PROTOCOL.md
README.md
src/release.rs
src/updater.rs
tests/bleeding.rs
```

Also update these if runtime release behavior changes:

```text
Cargo.toml
src/kernel.rs
RELEASE_NOTES_vX.Y.Z.md
```

## Privacy promise

phase1 update logic should never expose real emails, passwords, account identifiers, tokens, cookies, SSH keys, or recovery codes. Keep examples generic and redact host command output before showing it in the terminal.
