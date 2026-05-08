# phase1 security model

phase1 is an educational userspace simulator, not a hardened host sandbox. The simulator is designed to keep its virtual filesystem, process table, scheduler, audit log, boot state, and host-facing helpers separate from the real host operating system.

## Secure default and trust gate

phase1 defaults to safe mode with the host trust gate off. In the default launch, host-backed execution and host-network inspection are blocked.

Recommended launch:

```bash
cargo run
```

Equivalent explicit safe launch:

```bash
PHASE1_SAFE_MODE=1 cargo run
```

To intentionally test guarded host-backed runtimes while keeping the safe boot profile enabled, enable only the host trust gate:

```bash
PHASE1_SAFE_MODE=1 PHASE1_ALLOW_HOST_TOOLS=1 cargo run
```

or leave the shield enabled in the preboot selector and press `t` to enable trusted host tools.

This mode allows guarded runtime and read-only host-inspection commands such as:

- `browser`
- `ping`
- `wifi-scan`
- `python` / `py`
- `gcc` / `cc`
- `lang run ...`
- Python plugins
- read-only host network-interface inspection

Safe mode still blocks privileged host mutation. Mutating host actions require both safe mode off and the trust gate on:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 cargo run
```

Network mutation, such as an actual `wifi-connect`, additionally requires:

```bash
PHASE1_ALLOW_HOST_NETWORK_CHANGES=1
```

Only enable trusted host tools when you trust the commands, code, plugins, URLs, and network actions being tested.

## Host isolation boundaries

Safe mode is designed to prevent accidental host mutation, but phase1 is still a normal userspace program. It is not a virtualization boundary and should not be treated like a production sandbox or chroot.

Current defensive controls:

- Safe mode is on by default.
- Host trust is off by default.
- Guarded runtime execution can run with safe mode still enabled when `PHASE1_ALLOW_HOST_TOOLS=1` is set.
- Privileged host mutations still require `PHASE1_SAFE_MODE=0` plus the trust gate.
- Language source comes from the phase1 VFS or explicit inline input and is copied into a temporary guarded workspace.
- Guarded language commands receive a temporary `HOME`/`TMPDIR`, bounded stdin, bounded stdout/stderr, promptless host-tool environment variables, audit events, and command timeouts.
- `lang run` supports `--stdin`, `--stdin-file`, `--timeout`, and `PHASE1_LANG_TIMEOUT`; runtime timeout is capped.
- Browser fetches are restricted to `http://` and `https://` through `curl` protocol restrictions, timeout, and download-size limits.
- `ping` validates host text before invoking the host command.
- `wifi-connect` remains dry-run unless `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` is explicitly set.
- `phase1-storage` mutating Git/Rust actions require both `PHASE1_SAFE_MODE=0` and `PHASE1_ALLOW_HOST_TOOLS=1`.
- Runtime files and common secret material are ignored by Git.

## Redaction, history, and screenshot safety

Phase1 uses a central redaction policy for local operational output paths. The policy redacts common credential material including GitHub token prefixes, bearer/authorization headers, password/token/secret/API key assignments, CLI secret flags, URL credentials, cookies, sessions, and private-key blocks.

The ops log records structured command summaries instead of raw command lines by default. Sensitive commands are summarized as redacted events. `opslog tail` also redacts when reading back local logs.

The dashboard no longer displays the raw latest audit event. It displays a safe audit summary count so screenshots do not accidentally expose paths, command names, process names, or future sensitive strings.

Persistent shell history is sanitized before writing. The `phase1.learn` learning memory is also local, bounded, sanitized, and ignored by Git.

## Data that must never be committed

The repository ignores local runtime files and common credential material, including:

- `phase1.conf`
- `phase1.state`
- `phase1.history`
- `phase1.learn`
- `.phase1_history`
- `.env` and `.env.*`
- private keys such as `*.pem`, `*.key`, `id_rsa*`, and `id_ed25519*`
- files with token, secret, or password naming patterns

Do not paste GitHub tokens, passwords, API keys, SSH keys, recovery codes, session cookies, Apple ID credentials, email passwords, or other account secrets into phase1 commands.

## Host-backed commands

Some phase1 commands can call host tools when the trust gate is enabled:

- `browser` uses `curl` for HTTP/HTTPS text fetching.
- `ping`, `wifi-scan`, `ifconfig`, `iwconfig`, and `nmcli` can inspect host network state.
- `python`, `gcc`, plugin commands, and `lang run` can execute host programs through guarded runtime controls.
- `phase1-storage` can run guarded Git and Rust commands when explicitly trusted; mutating actions also require safe mode off.
- `wifi-connect` is dry-run unless safe mode is off and `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` is set.

Treat host-backed commands as trusted-user tools only. Do not run untrusted code or commands from someone else until a stronger OS-level sandbox backend is added.

## Persistent state and history

Persistent state stores phase1-managed virtual `/home` content in `phase1.state`. This file is local runtime state and is ignored by Git.

Persistent state should not be used for secrets. If a secret is accidentally written into the virtual `/home`, remove `phase1.state` before sharing logs, screenshots, or release assets.

Persistent shell history supports `PHASE1_HISTORY=off` and is ignored by Git as `phase1.history` / `.phase1_history`.

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
cargo run -p xtask -- validate
sh scripts/quality-check.sh full
```

Also search for accidental secrets before release:

```bash
git grep -n -I -E "(ghp_|github_pat_|BEGIN .*PRIVATE KEY|password|secret|access_token|api_key|oauth|cookie|session)"
```

If anything sensitive was ever committed, rotate the secret immediately. Removing it from the current tree is not enough because Git history may retain it.
