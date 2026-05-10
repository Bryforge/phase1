# Phase1 Updater Edge Stable Target Evidence

Status: updater fix validated
Date: 2026-05-10
Scope: in-system updater target and build validation

## Summary

The in-system updater now targets `origin/edge/stable` for the latest bleeding-edge update path.

The updater also prints the resolved target branch during execute mode so live logs clearly show the update target before git or cargo work continues.

## Fixed Behavior

- `update now --trust-host` targets `origin/edge/stable`.
- `cargo build --release` uses the longer build timeout path.
- Updater environment-mutating tests are serialized.
- Execute output prints `target     : origin/edge/stable`.
- Tracked local changes still block update execution before mutation.

## Final Live Evidence

```text
phase1 updater // execute latest bleeding edge
target     : origin/edge/stable
git fetch           [ok]
git checkout        [ok]
git pull --ff-only  [ok]
cargo build --release [ok]
build path : target/release/phase1
update: complete; exit and relaunch phase1 to run the updated code
```

## Validated Commands

```text
cargo fmt --all --check
cargo test -p phase1 --bin phase1 updater
cargo test -p phase1 --test smoke
cargo build --release
update now --trust-host live run
```

## Non-Claims

- No installer readiness claim
- No hardware validation claim
- No daily-driver claim
- No destructive disk writes
- No real-device write path
