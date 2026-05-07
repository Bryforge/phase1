# Phase1 Security Policy

Phase1 is an educational userspace simulator, not a hardened host sandbox. It is designed to keep its virtual filesystem, process table, scheduler, audit log, boot state, and command capability model separate from the real host operating system unless the operator deliberately enables trusted host tooling.

## Secure default

Phase1 defaults to SHIELD / safe mode. If no environment variable or saved boot config disables it, host-backed execution and host-network inspection are blocked.

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
- storage helper mutations
- Git helper mutations
- Rust host execution
- host network-interface inspection beyond a simulated loopback view

## Host tooling opt-in protocol

Host-backed commands require both safe mode off and host tools explicitly enabled:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 cargo run
```

Only do this when you trust the machine, repository, commands, code, plugins, URLs, and network actions being tested.

Host network mutation is a separate gate. WiFi connection changes require:

```bash
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 PHASE1_ALLOW_HOST_NETWORK_CHANGES=1 cargo run
```

Never paste secrets into Phase1 commands. Do not put credentials in clone URLs. Prefer HTTPS remotes without embedded credentials or a local SSH agent managed outside Phase1.

## Host isolation boundaries

Safe mode is designed to prevent accidental host interaction, but Phase1 is still a normal userspace program. It is not a virtualization boundary and should not be treated like a production sandbox.

Current defensive controls:

- Safe mode is on by default.
- Host tools require explicit trust opt-in.
- Host network discovery is skipped in safe mode and shows loopback only.
- Host-backed commands use argument lists rather than shell string interpolation.
- Browser fetches are restricted to `http://` and `https://` through `curl` protocol restrictions, timeout, and download-size limits.
- `ping` validates host text before invoking the host command.
- `wifi-connect` remains dry-run unless `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` is explicitly set.
- Python, C compiler, and plugin execution use timeouts and are disabled in safe mode.
- The ops log redacts credential-like strings, auth headers, token-like values, and URL credentials before writing.
- Runtime files and common secret material are ignored by Git.
- CI runs a dependency-free secret scan before the Rust validation gate.

## Command capability protocol

Commands declare capability metadata such as:

- `fs.read`
- `fs.write`
- `host.exec`
- `host.net`
- `net.admin`
- `wasm.exec`
- `sys.audit`

Use these commands inside Phase1 to inspect the current guard state:

```text
security
capabilities
sandbox
```

Review new commands against their capability label before merging. Any host-backed command must include explicit opt-in, argument-vector execution, no shell interpolation, timeout, stdin policy, bounded output, auditability, and redaction.

## Data that must never be committed

The repository ignores local runtime files and common credential material, including:

- `phase1.conf`
- `phase1.state`
- `.phase1_history`
- `phase1.history`
- `phase1.workspace/`
- `.env`, `.env.*`, and `.envrc`
- package manager credential files such as `.npmrc`, `.pypirc`, and `.cargo/credentials*`
- private keys such as `*.pem`, `*.key`, `id_rsa*`, and `id_ed25519*`
- keyrings such as `*.gpg`, `*.asc`, and `*.kdbx`
- files with token, secret, credential, or password naming patterns

Do not paste GitHub tokens, passwords, API keys, SSH keys, recovery codes, session cookies, Apple ID credentials, email passwords, or other account secrets into Phase1 commands.

## Repository security gates

GitHub Actions runs:

```bash
bash scripts/security-scan.sh
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

The workflow uses least-privilege `contents: read` permissions, does not persist checkout credentials, and has a job timeout.

`scripts/security-scan.sh` blocks high-confidence matches for:

- private key blocks
- GitHub, GitLab, OpenAI, Slack, and AWS token-like values
- hard-coded `password=`, `secret=`, `token=`, `api_key=`, and `client_secret=` assignments

This scanner is intentionally lightweight and dependency-free. It is not a replacement for GitHub Advanced Security, push protection, or a dedicated scanner such as Gitleaks, TruffleHog, or GitHub secret scanning.

## Persistent state and history

Persistent state stores Phase1-managed virtual `/home` content in `phase1.state`. This file is local runtime state and is ignored by Git.

Persistent state should not be used for secrets. If a secret is accidentally written into virtual `/home`, remove `phase1.state` before sharing logs, screenshots, or release assets.

## Account safety rules

Phase1 should never need your GitHub password, personal access token, SSH private key, browser cookies, Apple ID, email password, or recovery codes.

If a command, plugin, or copied instruction asks for account credentials, do not run it inside Phase1. Use official account security pages instead.

## Reporting vulnerabilities

Do not open a public issue for a real vulnerability that includes exploit details, tokens, private keys, or other sensitive data.

Report security concerns privately to the repository owner/maintainer through GitHub account contact methods or a private channel already established with the maintainer. Include:

- affected version or commit
- impacted command, file, or workflow
- expected security boundary
- observed behavior
- minimal reproduction steps without real secrets
- suggested remediation, if known

## Out of scope

The following are expected properties of the current simulator and are not vulnerabilities by themselves:

- Phase1 is not a production OS kernel.
- Simulated users, `/etc/passwd`, process IDs, CR3/CR4 values, PCIe devices, and VFS permissions are educational models.
- Host-backed commands are allowed when the operator explicitly disables safe mode and enables trusted host tools.

## Maintainer checklist

Before publishing a release or merging security-sensitive changes:

1. Confirm no secrets are present with `bash scripts/security-scan.sh`.
2. Run the full Rust quality gate.
3. Review new host command paths for: explicit opt-in, argument-vector execution, no shell interpolation, timeout, stdin policy, bounded output, auditability, and redaction.
4. Review persistence changes for scope: only Phase1-owned files should be written.
5. Review network changes for the separate `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` mutation gate.
6. Keep branch protection enabled so `Rust CI / fmt-check-clippy-test-security` is required before merge.

If anything sensitive was ever committed, rotate the secret immediately. Removing it from the current tree is not enough because Git history may retain it.
