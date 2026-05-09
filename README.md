# Phase1

<p align="center">
  <a href="https://bryforge.github.io/phase1/">
    <img src="assets/phase1-banner.jpeg" alt="Phase1 neon rainbow advanced operator kernel logo" width="780">
  </a>
</p>

<p align="center">
  <strong>secure · private · powerful · open</strong><br>
  A terminal-first Rust virtual OS console for operators, builders, and learners who want control.
</p>

<p align="center">
  <a href="https://bryforge.github.io/phase1/"><strong>Open the website</strong></a>
  ·
  <a href="https://github.com/Bryforge/phase1#quick-start"><strong>Quick start</strong></a>
  ·
  <a href="FEATURE_STATUS.md">Feature status</a>
  ·
  <a href="PHASE1_NATIVE_LANGUAGE.md">Fyr language</a>
  ·
  <a href="LEARNING.md">Learning system</a>
  ·
  <a href="QUALITY.md">Quality</a>
  ·
  <a href="base1/README.md">Base1</a>
  ·
  <a href="EDGE.md">Edge</a>
</p>

<p align="center">
  <img alt="Stable" src="https://img.shields.io/badge/stable-v4.2.0-39ff88">
  <img alt="Previous stable" src="https://img.shields.io/badge/previous%20stable-v4.1.0-7f8cff">
  <img alt="Edge" src="https://img.shields.io/badge/edge-v4.3.0--dev-00d8ff">
  <img alt="Rust" src="https://img.shields.io/badge/language-Rust-ff8a00">
  <img alt="Fyr" src="https://img.shields.io/badge/native%20language-Fyr-ff5a00">
  <img alt="Security" src="https://img.shields.io/badge/safe%20mode-default%20on-39ff88">
  <img alt="License" src="https://img.shields.io/badge/license-GPL--3.0-8a5cff">
</p>

<p align="center">
  <a href="PHASE1_NATIVE_LANGUAGE.md">
    <img src="assets/fyr-flame.svg" alt="Fyr flame mark with fyr written inside the flame" width="520">
  </a>
</p>

## What is Phase1?

Phase1 is a Rust-built, terminal-first virtual operating-system console created by Chase Bryan / Bryforge. It presents a futuristic operator surface while staying grounded in practical systems ideas: a simulated kernel, virtual filesystem, process table, audit log, command metadata, guarded host access, documentation, storage tooling, runtime planning, and a local learning layer.

It is designed to look and feel like a serious cyber-console without pretending to be magic. Phase1 is inspectable, teachable, and safe by default.

## Why it exists

Phase1 is built around a simple public promise:

> Give operators a beautiful terminal-first environment for learning, building, inspecting, and experimenting with system concepts while keeping host access explicit and guarded.

The project image is modern, neon, technical, and disciplined: advanced visuals, conservative security claims, repeatable validation, and clear separation between stable releases and experimental edge work.

## Fyr native language

Fyr is the Phase1-native language target for VFS automation, self-construction, and operator-owned scripts. It is designed around C-style explicit control, Rust-style safety posture, and Python-style readability while staying owned by Phase1.

Start with [`PHASE1_NATIVE_LANGUAGE.md`](PHASE1_NATIVE_LANGUAGE.md), then follow the dedicated [`Fyr roadmap`](docs/fyr/ROADMAP.md). The README/post flame mark lives at [`assets/fyr-flame.svg`](assets/fyr-flame.svg).

First working script inside Phase1:

```text
echo 'fn main() -> i32 { print("Hello, hacker!"); return 0; }' > hello_hacker.fyr
fyr run hello_hacker.fyr
```

Expected output:

```text
Hello, hacker!
```

## Core capabilities

| Area | What Phase1 provides |
| --- | --- |
| Operator console | A command-first interface with boot flow, dashboard, prompt, help, manual pages, and autocomplete. |
| Virtual OS model | Simulated kernel, VFS, process table, `/proc`, `/dev`, `/var/log`, architecture and system inspection commands. |
| Safe execution model | Safe mode on by default, host tools gated behind explicit trust, command capability metadata, and secret redaction. |
| Fyr native language | Phase1-owned `.fyr` language path with initial command runner and roadmap for self-construction workflows. |
| Learning system | `phase1-learn` stores local sanitized memory, imports history, learns notes/rules, and suggests next actions. |
| Website and wiki | Public GitHub Pages site, browser terminal demo, docs hub, tutorials, and roadmap material. |
| Storage and runtimes | Guarded storage/Git helper, Rust workflows, and a roadmap for broader programming-language support. |
| Base1 foundation | Planned secure host foundation for Raspberry Pi and ThinkPad X200-class systems. |
| Quality system | Scorecards, smoke tests, release metadata checks, CI workflows, CodeQL, and repeatable validation scripts. |

## Implementation status

Phase1 separates implemented features from experimental host integrations and future plans. The canonical matrix is [`FEATURE_STATUS.md`](FEATURE_STATUS.md).

| Feature area | Status | Short answer |
| --- | --- | --- |
| Terminal shell, VFS, process table, audit log, `/proc`, text tools, and dashboards | Implemented | These are simulated Phase1 subsystems and are covered by tests/smoke checks. |
| Local learning memory | Implemented | Local, sanitized, bounded, and git-ignored. |
| WASI-lite plugins | Implemented | Phase1's sandboxed plugin path; no host shell/network passthrough. |
| Python/Git/Cargo/Rust host-backed workflows | Experimental | Useful local integrations, but not hardened secure execution. |
| Host network/admin mutation | Restricted | Requires explicit trust gates and safe-mode changes. |
| Hardened VM/chroot/container sandbox | Not planned | Use a real VM/container for hostile code. |
| Full OS replacement | Not planned | Phase1 is a virtual OS console, not a replacement for Linux/macOS/Windows. |

Inside Phase1, run `capabilities` to inspect command-level gates and guard status. Use `FEATURE_STATUS.md` when you need the broader project-level answer to “is this implemented yet?”

## Quick start

Fresh clone, simplest launch:

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
sh phase1
```

After the file is executable, you can also run:

```bash
./phase1
```

Install a local `phase1` terminal command on macOS/Linux:

```bash
sh scripts/install-phase1-command.sh
phase1
```

Useful startup checks:

```bash
sh phase1 version
sh phase1 doctor
sh phase1 selftest
```

Rust-native launch remains available:

```bash
cargo run
```

Inside Phase1, start with:

```text
help
capabilities
sysinfo
security
wiki-quick
version --compare
roadmap
```

## Latest version check

Stable is currently `v4.2.0`. The next development package line is `v4.3.0-dev`.

To update your local checkout and see the active package version:

```bash
git fetch origin
git pull --ff-only origin master
sh phase1 version
```

Use stable release tags or release branches when you want the safest repository state. Use `v4.2.0` for the stable public representation. Use `v4.3.0-dev` only for future development and experimental polish.

## Smart local learning

Phase1 includes a local-first learning companion:

```bash
cargo run --bin phase1-learn -- status
cargo run --bin phase1-learn -- import-history
cargo run --bin phase1-learn -- suggest
```

Teach it project knowledge:

```bash
cargo run --bin phase1-learn -- teach deploy = use main for GitHub Pages deploys
cargo run --bin phase1-learn -- ask deploy
```

The learning memory is local, sanitized, bounded, and ignored by git. It does not call a cloud model or upload data. See [`LEARNING.md`](LEARNING.md).

## Release tracks

| Track | Version | Purpose |
| --- | --- | --- |
| Stable | `v4.2.0` | Current stable line for release-qualified work. |
| Previous stable | `v4.1.0` | Preserved previous stable release point. |
| Edge | `v4.3.0-dev` | Experimental development branch beyond stable. |
| Compatibility base | `v3.6.0` | Historical comparison base for compatibility references. |
| Base1 | `foundation` | Secure host design for real hardware targets. |

Use stable release tags or release branches when you want the safest repository state. Use edge branches only for active development and experimental polish.

## Public website

The public face of Phase1 lives at:

```text
https://bryforge.github.io/phase1/
```

The site presents the project as a polished neon/cyber operator system: animated space visuals, Phase1 branding, browser terminal demo, founder profile, sponsor path, wiki links, and mobile-first public documentation.

## Project structure

```text
src/                  Phase1 shell, kernel model, commands, UI, browser, runtime surfaces
src/bin/              Helper binaries including phase1-storage, phase1-install, phase1-learn
phase1-core/          Core package workspace member
xtask/                Repository validation helper
base1/                Secure host foundation docs and scripts
docs/wiki/            Manual and tutorial source
scripts/              Quality, runtime, Base1, wiki, and learning helpers
.github/workflows/    CI, CodeQL, Pages, and quality automation
```

## Quality and validation

Run the quick repository gate:

```bash
sh scripts/quality-check.sh quick
```

Run the full validation gate before release work:

```bash
sh scripts/quality-check.sh full
```

Rust-specific validation:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Optional security tooling:

```bash
cargo install cargo-audit --locked
cargo install cargo-deny --locked
cargo audit
cargo deny check
```

CI validates formatting, workspace checks, tests, quality rules, security workflow posture, and release metadata on pull requests and protected branch pushes.

## Base1 secure host foundation

Base1 is the planned real-hardware host layer below Phase1. Its purpose is to keep the host bootable, recoverable, and protected while Phase1 runs as a contained workload.

Start here:

- [`base1/README.md`](base1/README.md) — Base1 overview
- [`base1/SECURITY_MODEL.md`](base1/SECURITY_MODEL.md) — security model and boundary
- [`base1/HARDWARE_TARGETS.md`](base1/HARDWARE_TARGETS.md) — Raspberry Pi and X200 target matrix
- [`base1/PHASE1_COMPATIBILITY.md`](base1/PHASE1_COMPATIBILITY.md) — compatibility contract
- [`base1/ROADMAP.md`](base1/ROADMAP.md) — staged roadmap

First safe check:

```bash
sh scripts/base1-preflight.sh
```

The preflight checker is read-only.

## Runtime and host-backed features

Phase1 defaults to a guarded posture. Some host-backed features require explicit trust and safe-mode changes.

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

Do this only when you understand the host boundary.

## Safety model

Phase1 should never need your GitHub password, personal access token, SSH private key, browser cookies, Apple ID, email password, recovery codes, or private credentials.

Host-backed commands are explicit and guarded. Runtime files such as `phase1.state`, `phase1.history`, `phase1.learn`, and `phase1.log` are local operational artifacts. Command history, learning memory, and ops logs are sanitized before storage.

Security claims stay conservative until they are backed by repeatable builds, tests, audits, and hardware validation.

## Contributing

Phase1 values useful engineering over hype. Good contributions improve clarity, safety, documentation, validation, mobile fit, terminal usability, runtime support, or Base1 compatibility.

Before opening release-facing work, run:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
sh scripts/quality-check.sh quick
```

## License

Phase1 is released under GPL-3.0-only.

<!-- phase1:auto:repo-model:start -->
## Phase1 repository model

- `base/v4.2.0` is the frozen stable base.
- `edge/stable` is the active default development path.
- `checkpoint/*` branches are verified milestone snapshots.
- `feature/*` branches target `edge/stable`.

Keep the 4.2.0 image and stable base boring. Move tested work through edge/stable.
<!-- phase1:auto:repo-model:end -->

<!-- phase1:auto:current-status:start -->
## Current development status

- Current edge version: `v4.3.0-dev`
- Stable base: `base/v4.2.0`
- Active path: `edge/stable`
- Docs are generated by `scripts/update-docs.py`.
<!-- phase1:auto:current-status:end -->

