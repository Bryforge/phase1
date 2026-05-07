# Phase1

<p align="center">
  <img src="assets/phase1-banner.svg" alt="Phase1 neon advertisement logo" width="760">
</p>

<p align="center">
  <strong>Terminal-first virtual OS / advanced operator console in Rust.</strong><br>
  Simulated kernel. VFS. Process table. Audit log. Guarded browser. Secure-by-default shell.
</p>

<p align="center">
  <a href="docs/wiki/Home.md">User Manual</a> ·
  <a href="docs/wiki/11-Tutorials.md">Tutorials</a> ·
  <a href="docs/wiki/04-Command-Manual.md">Command Manual</a> ·
  <a href="docs/wiki/06-Browser-and-Networking.md">Browser and Network Guide</a>
</p>

![Edge](https://img.shields.io/badge/edge-v3.10.9--dev-00d8ff) ![Stable](https://img.shields.io/badge/stable-v3.10.7-39ff88) ![Rust](https://img.shields.io/badge/language-Rust-ff8a00) ![Security](https://img.shields.io/badge/default-safe%20mode%20on-39ff88)

Phase1 is a Rust-built, terminal-first educational virtual operating-system console. It models practical OS and cybersecurity concepts inside a safe userspace simulator: boot profiles, a virtual kernel, a VFS, process scheduling, `/proc`, `/dev`, `/var/log`, guarded networking, command capability metadata, pipelines, update tooling, runtime management, and a guarded terminal browser.

Phase1 is designed to feel like a futuristic operator console while staying intentionally safe by default.

## Status

| Track | Version | Notes |
| --- | --- | --- |
| Edge | `v3.10.9-dev` | Current `master` development build |
| Stable | `v3.10.7` | Current tagged stable release line |
| Compatibility base | `v3.6.0` | Historical stable comparison base |

The package version is the booted Phase1 version. Boot, ready line, `/proc/version`, dashboard, audit boot record, `/home/readme.txt`, and shutdown all dynamically reflect `CARGO_PKG_VERSION`.

## Quick start

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo run
```

At the boot selector, press `1` or `Enter`.

Inside Phase1:

```text
help
cat readme.txt
version --compare
security
sysinfo
roadmap
```

## Run checks

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

## Enable Python, browser, network inspection, and runtimes

Phase1 blocks host-backed tools by default. Use the runtime launcher when you want local Python, Rust, C, browser fetches, plugins, or host network inspection.

```bash
chmod +x scripts/phase1-runtimes.sh
./scripts/phase1-runtimes.sh
```

Manual boot equivalent:

```text
4    SHIELD off
t    TRUST HOST on
1    BOOT
```

Then try:

```text
py hello.py
lang support
browser example.com
ping example.com
```

## Manual and tutorials

The full manual lives in `docs/wiki/` and is structured for GitHub Wiki publishing.

| Page | Purpose |
| --- | --- |
| [User Manual](docs/wiki/Home.md) | Manual index and version matrix |
| [Quick Start](docs/wiki/01-Quick-Start.md) | First run and first commands |
| [Version Guide](docs/wiki/02-Version-Guide.md) | Edge, stable, and compatibility tracks |
| [Boot Modes and Security](docs/wiki/03-Boot-Modes-and-Security.md) | SHIELD, TRUST HOST, VAULT, and host gates |
| [Command Manual](docs/wiki/04-Command-Manual.md) | Organized command reference |
| [Files, Editors, and Pipelines](docs/wiki/05-Files-Editors-and-Pipelines.md) | VFS, AVIM, and pipelines |
| [Browser and Networking](docs/wiki/06-Browser-and-Networking.md) | Browser and network tutorials |
| [Language Runtimes](docs/wiki/07-Language-Runtimes.md) | Python, Rust, C, WASI-lite, runtime safety |
| [Updates and Releases](docs/wiki/08-Updates-Releases-and-Validation.md) | Validation, update, and release workflow |
| [Troubleshooting](docs/wiki/09-Troubleshooting.md) | Common failures and fixes |
| [Publish to GitHub Wiki](docs/wiki/10-Publish-to-GitHub-Wiki.md) | Native GitHub Wiki publishing steps |
| [Tutorials](docs/wiki/11-Tutorials.md) | Guided labs and workflows |

To publish the manual into the native GitHub Wiki once Wiki support is enabled:

```bash
chmod +x scripts/publish-wiki.sh
./scripts/publish-wiki.sh
```

## Core features

- Neo Tokyo terminal UI with laptop, desktop, ASCII, and color-aware modes
- Secure-by-default boot model with SHIELD and TRUST HOST gates
- Virtual filesystem with `/home`, `/proc`, `/dev`, `/tmp`, `/var/log`, `/etc`, and `/bin`
- Versioned in-system quick start at `/home/readme.txt`
- AVIM modal VFS editor
- Structured command chains and pipelines
- Simulated process table, audit log, and hardware commands
- Optimized guarded network inspection
- Guarded terminal browser with readable HTML extraction and link indexing
- Language runtime manager for major programming language families
- WASI-lite plugin/runtime support
- Update protocol and validation workflows
- Local operations log through `opslog`
- Idle-enter guard for mobile SSH terminals that send stale Enter events

## Safety

Phase1 is an educational simulator. It should never need your GitHub password, personal access token, SSH private key, browser cookies, Apple ID, email password, or recovery codes.

Host-backed commands are explicit and guarded. Runtime files such as `phase1.state`, `phase1.history`, and `phase1.log` are local operational artifacts. Do not publish them if they include sensitive local activity.

The Phase1 browser is a safe terminal reader. It does not run JavaScript, persist cookies, or store browser credentials.

## License

GPL-3.0
