# Phase1

<p align="center">
  <a href="https://bryforge.github.io/phase1/">
    <img src="assets/phase1-banner.svg" alt="Phase1 neon advertisement logo" width="760">
  </a>
</p>

<p align="center">
  <strong>Terminal-first virtual OS / advanced operator console in Rust.</strong><br>
  Simulated kernel. VFS. Process table. Audit log. Guarded browser. Secure-by-default shell.
</p>

<p align="center">
  <a href="https://bryforge.github.io/phase1/"><strong>Open the Phase1 website</strong></a>
  ·
  <a href="WIKI_ROADMAP.md">Website + wiki roadmap</a>
  ·
  <a href="base1/README.md">Base1 secure host foundation</a>
  ·
  <a href="TERMINAL.md">Phase1 Terminal</a>
</p>

![Stable](https://img.shields.io/badge/stable-v4.0.0-39ff88) ![Previous Stable](https://img.shields.io/badge/previous%20stable-v3.10.9-7f8cff) ![Rust](https://img.shields.io/badge/language-Rust-ff8a00) ![Security](https://img.shields.io/badge/default-safe%20mode%20on-39ff88) ![Base1](https://img.shields.io/badge/base1-secure%20host%20foundation-8a5cff)

Phase1 is a Rust-built, terminal-first educational virtual operating-system console. It models boot profiles, a virtual kernel, a VFS, process scheduling, `/proc`, `/dev`, `/var/log`, guarded networking, command capability metadata, pipelines, update tooling, runtime management, a guarded terminal browser, and a Base1 secure-host foundation.

Base1 is the planned secure hardware host foundation for Phase1 on Raspberry Pi and ThinkPad X200-class systems. Its mission is to keep the host bootable, recoverable, and protected if Phase1 is damaged, corrupted, or reset.

## Website

The main Phase1 homepage is designed for GitHub Pages:

```text
https://bryforge.github.io/phase1/
```

It uses a dark live-space background, moving rainbow visuals, the Phase1 neon logo, an interactive browser terminal demo, sponsor/founder sections, and a website/wiki implementation roadmap. Desktop browsing performance is protected with reduced canvas detail on large screens, debounced resize handling, hidden-tab animation pause, and reduced-motion support.

## Status

| Track | Version | Notes |
| --- | --- | --- |
| Stable | `v4.0.0` | Current stable release candidate |
| Previous stable | `v3.10.9` | Previous stable reference line |
| Compatibility base | `v3.6.0` | Historical comparison base |
| Base1 | `foundation` | Secure host design for Raspberry Pi and X200 targets |

The package version is the booted Phase1 version. Boot, ready line, `/proc/version`, dashboard, audit boot record, `/home/readme.txt`, and shutdown dynamically reflect `CARGO_PKG_VERSION`.

## Quick start

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo run
```

Inside Phase1:

```text
help
cat readme.txt
wiki
wiki-quick
version --compare
security
sysinfo
roadmap
```

## Phase1 Terminal

Phase1 Terminal is the dedicated launcher/profile layer for Linux and macOS. It installs the `phase1-terminal` command, loads Phase1-specific defaults, discovers the Phase1 checkout or binary, and can add native Linux/macOS launch integrations.

Install on Linux:

```bash
sh scripts/install-phase1-terminal-linux.sh
```

Install on macOS:

```bash
sh scripts/install-phase1-terminal-macos.sh
```

Then run:

```bash
phase1-terminal doctor
phase1-terminal
```

Full guide: [`TERMINAL.md`](TERMINAL.md).

## Base1 secure host foundation

Base1 is designed as the real-hardware host layer below Phase1. It treats Phase1 as a contained workload and keeps host boot files, host packages, host secrets, recovery paths, and security policy outside Phase1 control.

Start here:

- [`base1/README.md`](base1/README.md) - Base1 overview.
- [`base1/SECURITY_MODEL.md`](base1/SECURITY_MODEL.md) - threat model and security architecture.
- [`base1/HARDWARE_TARGETS.md`](base1/HARDWARE_TARGETS.md) - Raspberry Pi and X200 target matrix.
- [`base1/PHASE1_COMPATIBILITY.md`](base1/PHASE1_COMPATIBILITY.md) - Base1 and Phase1 compatibility contract.
- [`base1/ROADMAP.md`](base1/ROADMAP.md) - staged Base1 roadmap.

First safe checks:

```bash
sh scripts/base1-preflight.sh
```

The preflight checker is read-only. It reports readiness and warnings without changing the host.

## In-system wiki

Phase1 includes sandboxed WASI-lite manual pages readable from the prompt:

```text
wiki
wiki-quick
wiki-version
wiki-boot
wiki-commands
wiki-files
wiki-browse
wiki-lang
wiki-updates
wiki-trouble
wiki-tutorials
```

## Editors

`ned` is the quick line editor. It supports saving without quitting:

```text
ned notes.txt
:w      save
:wq     save and quit
.       save and quit
:q      quit without saving
```

`avim` is the advanced VFS editor:

```text
avim hello.py
```

Use `:help` inside `avim` for movement, edit, search, save, and quit commands.

## Stable v4 focus

- Preserve all v3.10.9 stable features.
- Improve editor usability and reliability.
- Improve logical wrapping for narrow terminals.
- Improve Linux color fallback for older systems such as ThinkPad X200 running Trisquel.
- Improve Raspberry Pi 5 default OS text and color compatibility.
- Add Base1 compatibility around secure Raspberry Pi and X200 host profiles.
- Improve the public website with clearer creator labeling, mobile readability, and desktop animation performance guards.
- Harden `phase1-storage` output redaction for common secret and URL-credential patterns.

## Run checks

Install local security tools once:

```bash
cargo install cargo-audit --locked
cargo install cargo-deny --locked
```

Then run the full quality gate:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

`cargo test --all-targets` includes unit tests plus scripted smoke tests for the main Phase1 shell, the guarded `phase1-storage` helper, the website, release metadata, and the Base1 secure host foundation files.

CI runs the same quality gate on pull requests, `master`, `main`, `release/**`, and manual dispatch. It also installs and runs RustSec advisory checks and dependency policy validation.

## Enable Python, browser, network inspection, and runtimes

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

## Manual and tutorials

The full manual lives in `docs/wiki/` and can be published to GitHub Wiki with:

```bash
chmod +x scripts/publish-wiki.sh
./scripts/publish-wiki.sh
```

The public website/wiki roadmap lives in [`WIKI_ROADMAP.md`](WIKI_ROADMAP.md).

## Safety

Phase1 is an educational simulator. It should never need your GitHub password, personal access token, SSH private key, browser cookies, Apple ID, email password, or recovery codes.

Host-backed commands are explicit and guarded. Runtime files such as `phase1.state`, `phase1.history`, and `phase1.log` are local operational artifacts.

Base1 is a secure host foundation, not a destructive installer. Its first tooling is intentionally read-only and compatibility-focused. Base1 security claims should remain conservative until backed by repeatable builds, audits, and hardware validation.

## License

GPL-3.0
