# phase1 security review

Review date: 2026-05-06

## Scope

This review covers the current phase1 source tree, runtime design, host-touching command paths, persistent state behavior, repository ignore rules, and release hygiene.

The goal is to reduce the chance that phase1 can compromise the host system, leak account information, or accidentally commit secrets while preserving practical usability for normal development, learning, validation, and local operator workflows.

## Security and usability goal

Phase1 should be as secure as possible while maintaining practical usability.

Security improvements should prefer safe defaults, explicit trust gates, redaction, dry-runs, read-only validation, and visible operator confirmation. They should not make legitimate local workflows unusable unless the restriction meaningfully reduces risk and is clearly documented.

Usability improvements should not bypass safe mode, host-trust gates, secret redaction, compatibility checks, or evidence-backed security claims.

## Security posture summary

phase1 is an educational userspace simulator. It is not a hardened VM, container, or OS sandbox. The safe design is to keep simulator state in memory or phase1-owned runtime files and to require intentional action before host-backed tools are used.

Current hardening state:

- Safe mode defaults to on.
- Host network discovery is skipped in safe mode.
- Safe mode shows a simulated loopback-only network view.
- Browser, ping, WiFi scan/connect, Python, C compiler, and plugin execution are blocked in safe mode.
- WiFi connection remains dry-run unless `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` is explicitly set.
- Persistent VFS state is limited to phase1-managed `/home` content in `phase1.state`.
- Runtime files and common credential files are ignored by Git.
- No obvious GitHub tokens, private keys, passwords, access tokens, API keys, cookies, sessions, or the authenticated account email were found by repository code search during this review.

## Threat model

### Protected assets

- GitHub account credentials and tokens
- SSH private keys
- Apple ID and email credentials
- Browser/session cookies
- Local files outside phase1 runtime files
- Host network configuration
- The user's GitHub repository integrity

### Primary risks

1. Accidental secret persistence in runtime files.
2. Accidental commit of local runtime files or credentials.
3. Host command execution through Python, C compiler, plugins, curl, ping, nmcli, networksetup, or similar tools.
4. Host network mutation through WiFi connection commands.
5. Saved boot config accidentally leaving host-backed mode enabled.
6. Running untrusted plugin code.

## Design decisions

### Safe mode default

Safe mode should be the default. A user should have to intentionally turn it off before host-backed commands can run.

Rationale: most phase1 functionality is educational and simulated. Host-backed tools are useful for demos and learning but are not required for normal safe operation.

### Host network inspection

In safe mode, network inspection should not call host tools such as `ifconfig`, `ip`, `networksetup`, `nmcli`, or WiFi scanners. It should show a simulated loopback view instead.

Rationale: host network metadata can reveal device names, interface addresses, MAC addresses, saved networks, nearby networks, or current network names.

### Host execution

Python, C compiler, and plugins must stay disabled in safe mode.

Rationale: these are direct host execution paths. Timeouts help reliability but are not a security boundary.

### Browser command

Browser must stay disabled in safe mode. When enabled, it should only allow `http://` and `https://`, use curl protocol restrictions, set a timeout, and cap download size.

Rationale: URL fetching can leak network metadata and fetch attacker-controlled content. Rendering is text-only, but host network access still matters.

### Persistent state

Persistent state should remain limited to phase1-managed virtual `/home` content and should be ignored by Git.

Rationale: persistence is useful, but it can store accidental secrets. It must not be treated as a secure secret store.

## Repository hygiene

The repository should ignore:

- `phase1.conf`
- `phase1.state`
- `.phase1_history`
- `.env` and `.env.*`
- private keys
- token/secret/password-named files
- temporary files and logs

A CI security scan should fail if obvious credential patterns or committed runtime state files appear in the tracked tree.

## Account information review

Repository code search was run for common secret/account patterns, including:

```text
ghp_
github_pat
BEGIN PRIVATE KEY
access_token
api_key
oauth
cookie
session
password
secret
token
bryforge@gmail.com
```

No matching committed source files were returned by the GitHub repository code search during this review.

Limitations:

- GitHub code search is not a full forensic history scan.
- It does not guarantee that a secret was never committed in unreachable history, deleted branches, local files, release assets, screenshots, or external systems.
- If any real credential is ever committed, rotate it immediately even if it is later deleted.

## Required validation before release

Run locally before publishing a release:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo test --test smoke -- --nocapture
```

Run secret scan before release:

```bash
git grep -n -I -E "(ghp_|github_pat_|BEGIN .*PRIVATE KEY|password|secret|access_token|api_key|oauth|cookie|session)"
```

Expected result: no real credentials. Documentation references to those words must be reviewed manually.

## Security recommendations

- Keep safe mode on for normal use.
- Do not run untrusted plugins.
- Do not paste account credentials into phase1.
- Do not share `phase1.state`, `.phase1_history`, logs, or screenshots without reviewing them for secrets.
- Keep host-backed command support as trusted-user functionality only.
- Consider a future stronger host-tools gate such as `PHASE1_ALLOW_HOST_TOOLS=1` in addition to disabling safe mode.
- Preserve practical usability while strengthening security controls; prefer guarded opt-in flows over removing useful workflows outright.
