# phase1

**phase1** is a terminal-first educational virtual operating-system console written in Rust.

It runs as a safe userspace simulator while modeling real OS concepts: a VFS with optional `/home` persistence, simulated process scheduler, syscall-style shell operations, `/proc`, `/dev`, `/var/log`, PCIe enumeration, network inspection, guarded host tools, plugins, generated man pages, command completion metadata, compact dashboard telemetry, and an in-memory audit log.

Current release: **v3.6.0**

## Highlights

- Mobile-friendly advanced operator console UI with automatic mobile-profile detection.
- Retro fastfetch-style preboot selector with colored `phase1` ASCII art.
- Configurable boot options for color, ASCII compatibility, safe mode, quick boot, mobile mode, persistent state, reboot, and quit.
- Optional persistent state mode saves and restores `/home` VFS content through `phase1.state`.
- Built-in `matrix` / `rain` digital-rain terminal effect.
- Compact `dash --compact` dashboard for release/demo screenshots.
- Registry-backed `help`, `man`, `complete`, aliases, and capability metadata.
- `capabilities` / `caps` command for command guard and policy visibility.
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

## Preboot configuration

On launch, phase1 opens a fastfetch-inspired boot selector before entering the shell. Press Enter to boot immediately, or toggle options by number/name:

```text
1 / boot        enter the main system
2 / color       toggle retro rainbow ANSI color
3 / ascii       toggle ASCII-compatible display mode
4 / safe        lock host integrations such as browser, ping, WiFi scan/connect, Python, C compiler, and plugins
5 / quick       skip the full boot matrix after selecting options
6 / mobile      force mobile-friendly layout defaults
p / persistent  toggle persistent state before entering the system
7 / reboot      restart the boot selector without entering the shell
8 / quit        abort boot and exit before the main system starts
9 / save        save the active boot profile to phase1.conf
0 / reset       reload default boot settings and remove saved boot config
```

When persistent state is on, phase1 loads `/home` content from `phase1.state` before the shell starts and saves `/home` changes back to `phase1.state` after shell commands. This keeps user-created VFS files available across runs while leaving system paths such as `/proc`, `/dev`, and `/var/log` freshly simulated each boot.

The selector automatically enables mobile mode for narrow terminals and common mobile terminal environments. It also honors environment defaults:

```bash
PHASE1_ASCII=1 cargo run
PHASE1_NO_COLOR=1 cargo run
PHASE1_SAFE_MODE=1 cargo run
PHASE1_QUICK_BOOT=1 cargo run
PHASE1_MOBILE_MODE=1 cargo run
PHASE1_PERSISTENT_STATE=1 cargo run
PHASE1_DEVICE=mobile cargo run
```

## Useful commands

```text
help
commands
complete p
capabilities
bootcfg
bootcfg show
bootcfg state
dash --compact
matrix 10
rain 5
man matrix
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

phase1 simulates the kernel, VFS, process table, and scheduler in memory. Optional persistent state only writes phase1-managed `/home` VFS content to the local `phase1.state` file. Some commands call host tools (`python3`, `cc`/`gcc`/`clang`, `curl`, `ping`, `nmcli`, `networksetup`). Those paths use validation and timeouts. The preboot safe profile disables host execution/network entry points from inside the shell. `wifi-connect` is dry-run by default and requires `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` before it attempts host network mutation.

## Roadmap designs

The roadmap design index is in `ROADMAP_DESIGNS.md`.

Detailed design tracks:

- `docs/roadmap/operator-shell.md`
- `docs/roadmap/virtual-kernel.md`
- `docs/roadmap/security-capabilities.md`
- `docs/roadmap/structured-pipelines.md`
- `docs/roadmap/package-plugin-runtime.md`
- `docs/roadmap/tui-dashboard.md`

## Release notes

Current release notes are in `RELEASE_NOTES_v3.6.0.md`.

## Development checks

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo audit
cargo deny check
```

For the end-to-end command smoke suite:

```bash
cargo test --test smoke -- --nocapture
```

## Roadmap

- Persistent shell history.
- Structured command output and pipelines.
- Capability enforcement based on command metadata.
- WASM/WASI plugin runtime.
- Full-screen TUI dashboard.
