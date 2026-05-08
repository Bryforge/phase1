# Version Guide

![Edge](https://img.shields.io/badge/edge-v4.1.0--dev-00d8ff) ![Stable](https://img.shields.io/badge/stable-v4.0.0-39ff88) ![Previous Stable](https://img.shields.io/badge/previous%20stable-v3.10.9-7f8cff) ![Base](https://img.shields.io/badge/compatibility-v3.6.0-7f8cff)

Phase1 has four version concepts that users may see.

| Name | Current value | Meaning |
| --- | --- | --- |
| Edge build | `v4.1.0-dev` | Bleeding-edge development line beyond v4.0.0 |
| Stable release | `v4.0.0` | Current stable release point and tag target |
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

## Edge release policy

Use an edge build when the version ends in `-dev` or when new behavior is still being validated.

Current edge example:

```text
v4.1.0-dev
```

Edge builds are best for testing post-v4.0.0 work, including terminal behavior, editor improvements, website changes, Base1 integration, guarded AI/plugin experiments, and documentation changes that follow active development.

## Stable release policy

Use a stable release when the version has no `-dev` suffix and has passed the full validation suite.

Current stable release point:

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

## Prepare the v4.0.0 tag

`release/v4.0.0` preserves the stable release point. Validate it before tagging:

```bash
git checkout release/v4.0.0
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
git tag v4.0.0
git push origin v4.0.0
```

## Continue bleeding-edge work

`edge/v4.1.0-dev` is the post-v4.0.0 development line:

```bash
git checkout edge/v4.1.0-dev
cargo metadata --no-deps --format-version 1 | grep '"version"'
```

Expected package version on the edge branch:

```text
4.1.0-dev
```
