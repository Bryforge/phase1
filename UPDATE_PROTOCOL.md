# phase1 update protocol

This file is the canonical update reference for phase1. Use it before changing update behavior, release metadata, release notes, or roadmap status.

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
3.7.2-dev   patch-level protocol hardening or follow-up bleeding-edge update
3.7.3-dev   patch-level protocol clarification without README churn
```

## Version bump rules

- **PATCH**, the third number, is required for every safe fix, documentation update, update-protocol improvement, formatting-only update, and incremental bleeding-edge feature.
- **MINOR**, the second number, is used when a roadmap track gains a meaningful new capability set.
- **MAJOR**, the first number, is reserved for deliberate compatibility-breaking behavior.
- Bleeding-edge builds keep the `-dev` suffix until they are promoted into a release.
- Stable release tags must use exact versions without `-dev`.
- Any follow-up change after a published bleeding-edge update must move the patch number before it is advertised.

## Current channels

```text
release        3.6.0
bleeding edge  3.7.3-dev
branch         master
stable branch  stable
```

## Safe update command behavior

The in-app updater must remain safe by default.

```text
update                         print the dry-run plan only
update protocol                 show versioning and safety reference
update bleeding --check         inspect local Git state only
update bleeding --execute       run guarded fetch/checkout/pull
update bleeding --execute --build
```

Execution rules:

- `update` without `--execute` must never modify files.
- `update --execute` must require `PHASE1_SAFE_MODE=0` and `PHASE1_ALLOW_HOST_TOOLS=1`.
- Host network mutation remains separately gated by `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1`.
- Tracked local changes must block update execution instead of being overwritten.
- Updater output must redact GitHub tokens, URL credentials, and private account-like values.
- The updater must not request private credentials, private keys, recovery codes, or browser cookies.

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

Patch-only bleeding-edge updates should avoid README churn. For PATCH updates, update only the files that carry the actual behavior, metadata, protocol, or tests:

```text
UPDATE_PROTOCOL.md
src/release.rs
src/updater.rs
tests/bleeding.rs
```

Update README.md only for MINOR or MAJOR releases, stable promotions, user-facing command changes that need public docs, or major roadmap wording changes.

Also update these if runtime release behavior changes:

```text
Cargo.toml
src/kernel.rs
RELEASE_NOTES_vX.Y.Z.md
```

## Privacy promise

phase1 update logic should never expose real emails, private account identifiers, tokens, cookies, SSH keys, or recovery codes. Keep examples generic and redact host command output before showing it in the terminal.
