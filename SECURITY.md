# phase1 security model

phase1 is an educational userspace simulator, not a hardened host sandbox. The simulator is designed to keep its virtual filesystem, process table, scheduler, audit log, and boot state separate from the real host operating system.

## Safe-by-default operating guidance

For maximum host and account safety, run phase1 with safe mode enabled and without host-network or host-execution override variables.

Recommended launch:

```bash
PHASE1_SAFE_MODE=1 cargo run
```

Safe mode is intended to prevent accidental use of host-backed commands such as browser, ping, WiFi scan/connect, Python, C compiler, and plugins.

## Data that must never be committed

The repository ignores local runtime files and common credential material, including:

- `phase1.conf`
- `phase1.state`
- `.phase1_history`
- `.env` and `.env.*`
- private keys such as `*.pem`, `*.key`, `id_rsa*`, and `id_ed25519*`
- files with token, secret, or password naming patterns

Do not paste GitHub tokens, passwords, API keys, SSH keys, recovery codes, or session cookies into phase1 commands.

## Host-backed commands

Some phase1 commands can call host tools when safe mode is disabled:

- `browser` uses `curl` for HTTP/HTTPS text fetching.
- `ping`, `wifi-scan`, `ifconfig`, `iwconfig`, and `nmcli` can inspect host network state.
- `python`, `gcc`, and plugin commands can execute host programs.
- `wifi-connect` is dry-run unless `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` is set.

Treat host-backed commands as trusted-user tools only. Do not run untrusted code or commands from someone else.

## Persistent state and history

Persistent state stores phase1-managed virtual `/home` content in `phase1.state`. This file is local runtime state and is ignored by Git.

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
