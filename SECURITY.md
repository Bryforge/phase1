# phase1 security model

phase1 is an educational userspace simulator, not a hardened host sandbox. The simulator is designed to keep its virtual filesystem, process table, scheduler, audit log, and boot state separate from the real host operating system.

## Secure default

phase1 now defaults to safe mode. If no environment variable or saved boot config disables it, host-backed execution and host-network inspection are blocked.

Recommended launch:

```bash
cargo run
```

Equivalent explicit safe launch:

```bash
PHASE1_SAFE_MODE=1 cargo run
```

Safe mode blocks or avoids host-backed behavior for:

- `browser`
- `ping`
- `wifi-scan`
- `wifi-connect`
- `python` / `py`
- `gcc` / `cc`
- Python plugins
- host network-interface inspection beyond a simulated loopback view

To intentionally test host-backed tools, disable safe mode at the preboot selector with option `4`, or launch with:

```bash
PHASE1_SAFE_MODE=0 cargo run
```

Only do this when you trust the commands, code, plugins, URLs, and network actions being tested.

## Host isolation boundaries

Safe mode is designed to prevent accidental host interaction, but phase1 is still a normal userspace program. It is not a virtualization boundary and should not be treated like a production sandbox.

Current defensive controls:

- Safe mode is on by default.
- Host network discovery is skipped in safe mode and shows loopback only.
- Host-backed commands are disabled in safe mode.
- Browser fetches are restricted to `http://` and `https://` through `curl` protocol restrictions, timeout, and download-size limits.
- `ping` validates host text before invoking the host command.
- `wifi-connect` remains dry-run unless `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` is explicitly set.
- Python, C compiler, and plugin execution use timeouts and are disabled in safe mode.
- Runtime files and common secret material are ignored by Git.

## Data that must never be committed

The repository ignores local runtime files and common credential material, including:

- `phase1.conf`
- `phase1.state`
- `.phase1_history`
- `.env` and `.env.*`
- private keys such as `*.pem`, `*.key`, `id_rsa*`, and `id_ed25519*`
- files with token, secret, or password naming patterns

Do not paste GitHub tokens, passwords, API keys, SSH keys, recovery codes, session cookies, Apple ID credentials, email passwords, or other account secrets into phase1 commands.

## Host-backed commands

Some phase1 commands can call host tools when safe mode is disabled:

- `browser` uses `curl` for HTTP/HTTPS text fetching.
- `ping`, `wifi-scan`, `ifconfig`, `iwconfig`, and `nmcli` can inspect host network state.
- `python`, `gcc`, and plugin commands can execute host programs.
- `wifi-connect` is dry-run unless `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` is set.

Treat host-backed commands as trusted-user tools only. Do not run untrusted code or commands from someone else.

## Persistent state and history

Persistent state stores phase1-managed virtual `/home` content in `phase1.state`. This file is local runtime state and is ignored by Git.

Persistent state should not be used for secrets. If a secret is accidentally written into the virtual `/home`, remove `phase1.state` before sharing logs, screenshots, or release assets.

Future persistent shell history must remain opt-in or explicitly tied to persistent state, must support `PHASE1_HISTORY=off`, and must be ignored by Git as `.phase1_history`.

## Account safety rules

phase1 should never need your GitHub password, personal access token, SSH private key, browser cookies, Apple ID, email password, or recovery codes.

If a command, plugin, or copied instruction asks for account credentials, do not run it inside phase1. Use official account security pages instead.

## Release checklist

Before publishing a release:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo test --test smoke -- --nocapture
```

Also search for accidental secrets before release:

```bash
git grep -n -I -E "(ghp_|github_pat_|BEGIN .*PRIVATE KEY|password|secret|access_token|api_key|oauth|cookie|session)"
```

If anything sensitive was ever committed, rotate the secret immediately. Removing it from the current tree is not enough because Git history may retain it.
