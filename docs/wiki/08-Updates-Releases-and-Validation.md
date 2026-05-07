# Updates, Releases, and Validation

![Update](https://img.shields.io/badge/update-dry%20run%20first-00d8ff) ![Validation](https://img.shields.io/badge/tests-required-39ff88) ![Release](https://img.shields.io/badge/release-tagged-ffcc00)

Phase1 treats updates and release work as guarded operator workflows. Dry-run plans are preferred until execution is explicitly requested.

## Validation checklist

Run this before every push:

> [!TIP]
> TRY THIS
>
> ```bash
> cargo fmt --all -- --check
> cargo check --all-targets
> cargo clippy --all-targets -- -D warnings
> cargo test --all-targets
> ```

Full release validation:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

Optional targeted validation:

```bash
cargo test --test smoke -- --nocapture
cargo test --test bleeding -- --nocapture
cargo test --test game -- --nocapture
```

## In-app update commands

| Command | Purpose |
| --- | --- |
| `update` | Show safe update plan |
| `update protocol` | Show update safety rules |
| `update latest --trust-host --check` | Check latest update with host trust gate |
| `update latest --trust-host --execute --build` | Execute guarded update and build |
| `update now --trust-host` | Run guarded update now workflow |
| `update test quick` | Show quick test plan |
| `update test full` | Show full test plan |
| `update test quick --trust-host --execute` | Execute quick test workflow with host trust |

> [!TIP]
> TRY THIS INSIDE PHASE1
>
> ```text
> update
> update protocol
> update test quick
> update test full
> ```

## Update safety rules

> [!IMPORTANT]
> Update execution is host-backed and guarded. The operator must explicitly request execution.

Rules:

- `update` defaults to a dry-run plan
- `--execute` is required before file mutation
- `--trust-host` is required for host-backed execution paths
- SHIELD must be off
- TRUST HOST must be on
- tracked local changes block execution instead of being overwritten
- updater output is sanitized before display

## Edge release workflow

Use edge versions for active development builds that still carry the `-dev` suffix.

Current edge version:

```text
v4.0.0-dev
```

Edge tagging example:

```bash
git status
git log -1 --oneline
git tag v4.0.0-dev
git push origin master
git push origin v4.0.0-dev
```

## Stable release workflow

Stable releases remove the `-dev` suffix and must pass validation.

Current stable release:

```text
v3.10.9
```

Stable promotion checklist:

1. Remove `-dev` from `Cargo.toml`.
2. Refresh `Cargo.lock`.
3. Update README and wiki version references.
4. Run full validation.
5. Commit the promotion.
6. Tag the release.
7. Push branch and tag.

Future v4 stable promotion example:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
git status
git add Cargo.toml Cargo.lock README.md docs/wiki plugins
git commit -m "Promote phase1 v4.0.0"
git tag v4.0.0
git push origin master
git push origin v4.0.0
```

## Documentation release checklist

When version numbers change, update:

```text
README.md
docs/wiki/Home.md
docs/wiki/02-Version-Guide.md
docs/wiki/08-Updates-Releases-and-Validation.md
docs/wiki/10-Publish-to-GitHub-Wiki.md
plugins/wiki-version.wasi
plugins/wiki-updates.wasi
/home/readme.txt generator if command behavior changed
```

## Tutorial: Run a complete release check

> [!TIP]
> TRY THIS
>
> ```bash
> git pull origin master
> cargo fmt --all -- --check
> cargo check --all-targets
> cargo clippy --all-targets -- -D warnings
> cargo test --all-targets
> cargo audit
> cargo deny check
> git status
> ```

Expected result:

```text
all tests pass
working tree clean
```

## Handling failures

| Failure | Action |
| --- | --- |
| Formatting diff | Run `cargo fmt --all`, then re-check |
| Compile error | Fix source, then run `cargo check --all-targets` |
| Unit test failure | Fix implementation or expected behavior |
| Smoke test failure | Compare expected output with current UI text |
| Local changes block pull | Commit, stash, or discard local changes before pulling |
