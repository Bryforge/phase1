# Phase1

<p align="center">
  <img src="assets/phase1-banner.svg" alt="Phase1 neon advertisement logo" width="760">
</p>

<p align="center">
  <strong>Terminal-first virtual OS / advanced operator console in Rust.</strong><br>
  Simulated kernel. VFS. Process table. Audit log. Guarded browser. Secure-by-default shell.
</p>

![Edge](https://img.shields.io/badge/edge-v4.0.0--dev-00d8ff) ![Stable](https://img.shields.io/badge/stable-v3.10.9-39ff88) ![Rust](https://img.shields.io/badge/language-Rust-ff8a00) ![Security](https://img.shields.io/badge/default-safe%20mode%20on-39ff88)

Phase1 is a Rust-built, terminal-first educational virtual operating-system console. It models boot profiles, a virtual kernel, a VFS, process scheduling, `/proc`, `/dev`, `/var/log`, guarded networking, command capability metadata, pipelines, update tooling, runtime management, and a guarded terminal browser.

## Status

| Track | Version | Notes |
| --- | --- | --- |
| Edge | `v4.0.0-dev` | Current `master` development build |
| Stable | `v3.10.9` | Current stable reference line |
| Compatibility base | `v3.6.0` | Historical comparison base |

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

`ned` is the quick line editor. It now supports saving without quitting:

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

## Compatibility focus for v4 edge

- Preserve all v3.10.9 stable features.
- Improve editor usability and reliability.
- Improve logical wrapping for narrow terminals.
- Improve Linux color fallback for older systems such as ThinkPad X200 running Trisquel.
- Improve Raspberry Pi 5 default OS text and color compatibility.

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

`cargo test --all-targets` includes unit tests plus scripted smoke tests for the main Phase1 shell and the guarded `phase1-storage` helper.

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

## Safety

Phase1 is an educational simulator. It should never need your GitHub password, personal access token, SSH private key, browser cookies, Apple ID, email password, or recovery codes.

Host-backed commands are explicit and guarded. Runtime files such as `phase1.state`, `phase1.history`, and `phase1.log` are local operational artifacts.

## License

GPL-3.0
