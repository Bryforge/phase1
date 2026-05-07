# Troubleshooting

![Troubleshooting](https://img.shields.io/badge/troubleshooting-operator%20guide-00d8ff) ![Safe](https://img.shields.io/badge/safe-default-39ff88)

This page covers common Phase1 build, boot, terminal, browser, network, runtime, and Git workflow issues.

## Build fails after pull

> [!TIP]
> TRY THIS
>
> ```bash
> git pull origin master
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
git pull --rebase origin master
```

Temporarily move local work aside:

```bash
git stash push -m "local phase1 changes"
git pull origin master
git stash pop
```

Discard local work:

```bash
git restore <file>
git pull origin master
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
| Disabled by safe boot | SHIELD is on | Use runtime launcher or press `4` at boot |
| Trusted host tools disabled | TRUST HOST is off | Use runtime launcher or press `t` at boot |
| Curl unavailable | Host lacks curl | Install curl on the host |
| URL rejected | Unsupported protocol or URL credentials | Use plain HTTP/HTTPS without embedded credentials |
| Empty readable output | Page is script-heavy | Try another URL or inspect with a full browser outside Phase1 |

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
| Safe-mode denial | SHIELD is on | Disable SHIELD only when intended |
| Host-tools denial | TRUST HOST is off | Enable TRUST HOST |
| WiFi scan unavailable | Host command unsupported | Use OS-native WiFi tools |
| WiFi connect blocked | Mutation gate off | Set `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` |

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
> git pull origin master
> cargo fmt --all
> cargo fmt --all -- --check
> cargo check --all-targets
> cargo test --all-targets
> cargo run
> ```

Expected result:

```text
format passes
compile passes
tests pass
Phase1 boots
shutdown banner reports the current package version
```
