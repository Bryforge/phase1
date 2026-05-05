# phase1

**phase1** is a terminal-first educational virtual operating-system console written in Rust.

It runs as a safe userspace simulator while modeling real OS concepts: an in-memory VFS, simulated process scheduler, syscall-style shell operations, `/proc`, `/dev`, `/var/log`, PCIe enumeration, network inspection, guarded host tools, plugins, generated man pages, command completion metadata, and an in-memory audit log.

Current release: **v3.5.0**

## Highlights

- Mobile-friendly operator console UI.
- Registry-backed `help`, `man`, and `complete` commands.
- Syscall-style kernel boundary for read, write, spawn, and kill paths.
- In-memory audit log exposed through `audit`.
- Safer host integrations with validation and timeouts.
- Browser allows only `http://` and `https://` and strips HTML into terminal text.
- Network commands use best-effort host discovery with honest fallbacks.
- VFS includes `/home`, `/proc`, `/dev`, `/tmp`, `/var/log`, `/etc`, and `/bin`.
- CI checks formatting, compilation, Clippy, tests, cargo-audit, and cargo-deny.

## Quick start

```bash
cargo run
```

or

```bash
cargo build --release
./target/release/phase1
```

## Useful commands

```text
help
complete p
man browser
ls -l /
cat /proc/version
spawn worker --background
jobs
ps
audit
echo "hello world" > note.txt
cat note.txt
browser phase1
browser https://example.com
```

## Safety model

phase1 simulates the kernel, VFS, process table, and scheduler in memory. Some commands call host tools (`python3`, `cc`/`gcc`/`clang`, `curl`, `ping`, `nmcli`, `networksetup`). Those paths use validation and timeouts. `wifi-connect` is dry-run by default and requires `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` before it attempts host network mutation.

## Roadmap designs

The roadmap design index is in `ROADMAP_DESIGNS.md`.

Detailed design tracks:

- `docs/roadmap/operator-shell.md`
- `docs/roadmap/virtual-kernel.md`
- `docs/roadmap/security-capabilities.md`
- `docs/roadmap/structured-pipelines.md`
- `docs/roadmap/package-plugin-runtime.md`
- `docs/roadmap/tui-dashboard.md`

## Development checks

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

## Roadmap

- Registry-backed alias dispatch.
- Persistent shell history.
- Structured command output and pipelines.
- Capability enforcement based on command metadata.
- WASM/WASI plugin runtime.
- Full-screen TUI dashboard.
