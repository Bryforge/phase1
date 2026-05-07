# Version Guide

![Stable](https://img.shields.io/badge/stable-v4.0.0-39ff88) ![Previous Stable](https://img.shields.io/badge/previous%20stable-v3.10.9-7f8cff) ![Base](https://img.shields.io/badge/compatibility-v3.6.0-7f8cff)

Phase1 has three release concepts that users may see.

| Name | Current value | Meaning |
| --- | --- | --- |
| Stable release | `v4.0.0` | Current tagged non-dev release line |
| Previous stable | `v3.10.9` | Previous stable reference line |
| Compatibility base | `v3.6.0` | Historical stable base used by compatibility comparisons |

## Runtime version behavior

The package version is the booted Phase1 version. Runtime surfaces dynamically reflect `CARGO_PKG_VERSION`.

```text
boot card version        v<CARGO_PKG_VERSION>
ready line               phase1 <CARGO_PKG_VERSION> ready
/proc/version            phase1 <CARGO_PKG_VERSION>
dmesg                    phase1 <CARGO_PKG_VERSION> boot
dash / dashboard          PHASE1 DASHBOARD v<CARGO_PKG_VERSION>
audit                    kernel.boot version=<CARGO_PKG_VERSION>
/home/readme.txt          reflects v<CARGO_PKG_VERSION>
exit / shutdown banner    shutdown: phase1 <CARGO_PKG_VERSION>
```

> [!TIP]
> TRY THIS
>
> ```text
> version
> version --compare
> cat /proc/version
> dmesg
> audit
> cat readme.txt
> ```

## Stable release policy

Use a stable release when the version has no `-dev` suffix and has passed the full validation suite.

Current stable release:

```text
v4.0.0
```

Stable builds are best for demos, README screenshots, public posts, tagged release notes, and normal users.

## Previous stable line

The previous stable reference line remains:

```text
v3.10.9
```

Use this when comparing the current stable behavior against the earlier stable series.

## Compatibility base

The long-term compatibility base remains:

```text
v3.6.0
```

It is used for historical comparison and compatibility language.

## Check the current package version

From the host:

```bash
cargo metadata --no-deps --format-version 1 | grep '"version"'
```

From inside Phase1:

```text
version
cat /proc/version
```

## Promote a release candidate to stable

> [!IMPORTANT]
> Only promote after formatting, Clippy, tests, audit, and dependency policy checks pass.

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

Then remove the `-dev` suffix from `Cargo.toml`, update `Cargo.lock`, update README/wiki/version references, run validation again, commit, and tag.

Current stable tagging example:

```bash
git status
git log -1 --oneline
git tag v4.0.0
git push origin master
git push origin v4.0.0
```
