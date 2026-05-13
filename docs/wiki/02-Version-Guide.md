# Version Guide

![Edge](https://img.shields.io/badge/edge-v6.0.0-00d8ff) ![Stable](https://img.shields.io/badge/stable-v5.0.0-39ff88) ![Previous Stable](https://img.shields.io/badge/previous%20stable-v4.4.0-7f8cff) ![Compatibility](https://img.shields.io/badge/compatibility-v3.6.0-7f8cff)

Phase1 uses several version labels because the project has a stable release line, an active edge line, historical compatibility references, and a long-term Base1 foundation track.

| Name | Current value | Meaning |
| --- | --- | --- |
| Edge build | `v6.0.0` | Active development line on `edge/stable`. Use it for current wiki, v6 UI/help, Base1 planning, Fyr work, and validation. |
| Stable release | `v5.0.0` | Current stable base for release-qualified public references. |
| Previous stable | `v4.4.0` | Previous stable line for compatibility checks and historical comparison. |
| Compatibility base | `v3.6.0` | Older comparison base retained for long-running compatibility language. |
| Base1 | `foundation` | Host-foundation track for boot, recovery, storage, rollback, installer, and hardware validation work. |

## Runtime version behavior

The package version is the booted Phase1 version. Runtime surfaces should reflect `CARGO_PKG_VERSION`.

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

## Edge policy

Use the edge line for active development and documentation that reflects the current default branch.

Current edge line:

```text
v6.0.0
```

Current active branch:

```text
edge/stable
```

Edge work may include UI improvements, help-system work, website polish, wiki updates, Base1 plans, recovery evidence, storage/rollback dry-runs, Fyr growth, and guarded host-integration experiments.

> [!IMPORTANT]
> Edge documentation must label experimental, host-backed, or future work clearly. Do not convert roadmap goals into release claims.

## Stable policy

Use stable references for public demos, release notes, screenshots, and safer checkpoints.

Current stable base:

```text
v5.0.0
```

Stable builds should not rely on unsupported edge-only behavior.

## Previous stable line

The previous stable reference line is:

```text
v4.4.0
```

Use this when comparing the current stable behavior against the earlier stable series.

## Compatibility base

The long-term compatibility base remains:

```text
v3.6.0
```

It is used for historical comparison and compatibility language.

## Base1 foundation status

Base1 is not a finished hardened OS. It is the host-foundation track that keeps boot, recovery, installer, rollback, storage, and hardware-validation claims separate from the Phase1 virtual OS console.

Current Base1 status:

```text
foundation
```

Start with [Base1 OS Track](13-Base1-OS-Track.md) before writing boot or recovery claims.

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

## Continue edge work

```bash
git fetch origin
git checkout edge/stable
cargo metadata --no-deps --format-version 1 | grep '"version"'
sh scripts/quality-check.sh quick
```

Expected package version on the current edge line:

```text
6.0.0
```

## Release-readiness rule

Before promoting or publishing a release-facing change, update version-sensitive docs and run validation:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
sh scripts/quality-check.sh quick
```

Optional release/security checks:

```bash
cargo audit
cargo deny check
```
