# Version Guide

![Edge](https://img.shields.io/badge/edge-v3.10.9--dev-00d8ff) ![Stable](https://img.shields.io/badge/stable-v3.10.7-39ff88) ![Base](https://img.shields.io/badge/compatibility-v3.6.0-7f8cff)

Phase1 has three version concepts that users may see.

| Name | Current value | Meaning |
| --- | --- | --- |
| Edge build | `v3.10.9-dev` | Current `master` development version |
| Stable release | `v3.10.7` | Latest tagged non-dev release line |
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

Use an edge release when the version ends in `-dev` or when new behavior is still being validated.

Examples:

```text
v3.10.8-dev
v3.10.9-dev
```

Edge builds are best for testing:

- browser improvements
- network stack changes
- terminal raw input changes
- new UI modes
- update workflow changes
- documentation changes that follow active development

## Stable release policy

Use a stable release when the version has no `-dev` suffix and has passed the full validation suite.

Example:

```text
v3.10.7
```

Stable builds are best for demos, README screenshots, public posts, and tagged release notes.

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

## Promote edge to stable

> [!IMPORTANT]
> Only promote after formatting and tests pass.

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

Then remove the `-dev` suffix from `Cargo.toml`, update `Cargo.lock`, update README/wiki version references, run validation again, commit, and tag.

```bash
git status
git log -1 --oneline
git tag v3.10.9
git push origin master
git push origin v3.10.9
```
