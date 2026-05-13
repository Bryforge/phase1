# Troubleshooting

![Troubleshooting](https://img.shields.io/badge/troubleshooting-operator%20guide-00d8ff) ![Safe](https://img.shields.io/badge/safe-default-39ff88) ![Edge](https://img.shields.io/badge/edge-v6.0.0-00d8ff)

This page covers common Phase1 build, boot, terminal, browser, network, runtime, wiki, Base1, Fyr, and Git workflow issues.

## Build fails after pull

> [!TIP]
> TRY THIS
>
> ```bash
> git fetch origin
> git checkout edge/stable
> git pull origin edge/stable
> cargo fmt --all
> cargo fmt --all -- --check
> cargo check --all-targets
> cargo test --all-targets
> ```

If `cargo fmt --all -- --check` prints a diff, run `cargo fmt --all` and then re-run validation.

## Local changes block a pull

Symptom:

```text
error: Your local changes to the following files would be overwritten by merge
```

Choose one path:

```bash
git status
```

Keep local work:

```bash
git add .
git commit -m "Save local work"
git pull --rebase origin edge/stable
```

Temporarily move local work aside:

```bash
git stash push -m "local phase1 changes"
git pull origin edge/stable
git stash pop
```

Discard local work:

```bash
git restore <file>
git pull origin edge/stable
```

## Safe mode blocks Python, browser, or runtimes

Symptoms:

```text
python: disabled by safe boot profile
browser: disabled by safe boot profile
python: disabled; set PHASE1_ALLOW_HOST_TOOLS=1 to enable trusted host tools
```

Fix:

```bash
./scripts/phase1-runtimes.sh
```

or at boot:

```text
4    SHIELD off
t    TRUST HOST on
1    BOOT
```

Verify:

```text
security
capabilities
```

## Browser fetch fails

Check host tools and gates:

```text
security
browser about
```

Possible causes:

| Symptom | Cause | Fix |
| --- | --- | --- |
| Disabled by safe boot | SHIELD is on | Use runtime launcher or press `4` at boot. |
| Trusted host tools disabled | TRUST HOST is off | Use runtime launcher or press `t` at boot. |
| Curl unavailable | Host lacks curl | Install curl on the host. |
| URL rejected | Unsupported protocol or URL credentials | Use plain HTTP/HTTPS without embedded credentials. |
| Empty readable output | Page is script-heavy | Try another URL or inspect with a full browser outside Phase1. |

## Network commands fail

Check:

```text
security
ifconfig
nmcli
iwconfig
```

Possible causes:

| Symptom | Cause | Fix |
| --- | --- | --- |
| Safe-mode denial | SHIELD is on | Disable SHIELD only when intended. |
| Host-tools denial | TRUST HOST is off | Enable TRUST HOST. |
| WiFi scan unavailable | Host command unsupported | Use OS-native WiFi tools. |
| WiFi connect blocked | Mutation gate off | Set `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1`. |

## Fyr script does not run

Check the file extension and command:

```text
ls
cat hello.fyr
fyr run hello.fyr
```

Common causes:

| Symptom | Cause | Fix |
| --- | --- | --- |
| File not found | Script is not in the current VFS directory | Run `pwd`, `ls`, or use the correct path. |
| Parse error | Example uses unsupported Fyr syntax | Start from the examples in `14-Fyr-Native-Language.md`. |
| No output | Script did not call `print` | Add a supported `print(...)` statement. |
| Unexpected behavior | Language surface changed | Re-run tests and update docs with implementation. |

## Base1 dry-run looks like an install

Base1 dry-run commands are previews and checks. They should not be described as completed installs.

Verify that commands include read-only or dry-run flags:

```bash
sh scripts/base1-x86_64-detect.sh --dry-run
sh scripts/base1-b2-assembly-dry-run.sh --dry-run --profile x86_64-vm-validation
sh scripts/base1-install-dry-run.sh --dry-run --target /dev/example
sh scripts/base1-recovery-dry-run.sh --dry-run
```

Use [Base1 OS Track](13-Base1-OS-Track.md) for correct public wording.

## Termius or mobile SSH sends a stale Enter

Phase1 includes an idle-enter guard for terminals that send an empty Enter after the phone wakes or switches back to the session.

The guard is enabled by default. It is designed to ignore suspicious stale empty Enter events after idle gaps.

If your terminal behaves normally and you want to disable the guard, use the documented environment setting from the current source once exposed in your build. If the issue persists, capture:

```text
terminal app
host OS
Phase1 version
steps that trigger the stale Enter
```

Then run:

```text
version
security
history status
```

## Persistent state looks old

If VAULT mode is enabled, `/home` persists to `phase1.state`. That can keep an old `readme.txt` or old lab files.

Fix options:

```bash
rm phase1.state
```

or boot with VAULT off.

Inside Phase1:

```text
cat readme.txt
```

The generated readme should reflect the current booted version when persistent state is fresh.

## Wiki page looks stale

Check whether you are viewing the reviewable source wiki, native GitHub Wiki, or website copy.

| Surface | Source |
| --- | --- |
| Reviewable source | `docs/wiki/` |
| Native GitHub Wiki | `Bryforge/phase1.wiki` after publishing |
| In-system compact wiki | `plugins/wiki-*.wasi` |
| Website links | `site/` and GitHub Pages output |

If the native GitHub Wiki is stale, publish the reviewed source:

```bash
scripts/publish-wiki.sh
```

## Smoke test expected output fails

Smoke tests often compare exact output labels. If UI wording changes, update tests intentionally.

> [!TIP]
> TRY THIS
>
> ```bash
> cargo test --test smoke -- --nocapture
> ```

Review the failing expected string and the actual output. Update either the implementation or the expected string based on intended behavior.

## Bleeding test cannot spawn Phase1

Symptom:

```text
spawn phase1: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Run:

```bash
cargo test --test bleeding -- --nocapture
```

Check that the test resolves the built binary path correctly and runs from an isolated temp directory.

## Dead code warning

Symptom:

```text
warning: method `render_text` is never used
```

Fix options:

- remove the unused method
- call the method if it should be part of the browser API
- mark intentionally retained helpers only when justified

## Clean validation sequence

> [!TIP]
> TRY THIS
>
> ```bash
> git fetch origin
> git checkout edge/stable
> git pull origin edge/stable
> cargo fmt --all
> cargo fmt --all -- --check
> cargo check --all-targets
> cargo test --all-targets
> sh scripts/quality-check.sh quick
> cargo run
> ```

Expected result:

```text
format passes
compile passes
tests pass
quality gate passes
Phase1 boots
shutdown banner reports the current package version
```
