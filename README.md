# Phase1

<p align="center">
  <img src="assets/phase1-banner.svg" alt="Phase1 minimal neon cyberdeck banner" width="820">
</p>

<p align="center">
  <strong>Terminal-first virtual OS / advanced operator console in Rust.</strong><br>
  Simulated kernel. VFS. Process table. Audit log. Secure-by-default shell. Futuristic Neo Tokyo UI.
</p>

<p align="center">
  <img src="assets/phase1-rainbow-logo.svg" alt="Phase1 rainbow neon app icon" width="180">
</p>

**Phase1** is a Rust-built, terminal-first educational virtual operating-system console. It runs as a safe userspace simulator while modeling real OS and cybersecurity concepts: boot profiles, a virtual kernel, a VFS, process scheduling, `/proc`, `/dev`, `/var/log`, PCIe-style hardware views, guarded networking, command capability metadata, shell history, pipelines, runtime management, update tooling, and local operations logging.

Phase1 is designed to feel like a futuristic hacker/operator console while staying intentionally safe by default.

## Current status

- Current package version on `master`: **v3.10.7**
- Latest tagged release: **v3.10.6**
- Current development channel: **bleeding-edge**
- Virtual kernel baseline: **v3.6.0**
- Default branch: **master**
- Language: **Rust**
- Default security posture: **safe mode on, host tools off**
- Version scheme: **MAJOR.MINOR.PATCH[-dev]**

The package/application version tracks the release line. The virtual kernel baseline remains `3.6.0` for compatibility reporting inside the simulated OS. Patch builds after a stable tag should be treated as bleeding-edge until promoted and tagged.

## Quick start

Clone and run:

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo run
```

Build a release binary:

```bash
cargo build --release
./target/release/phase1
```

Run the full validation suite:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

Run with host-backed language runtimes enabled:

```bash
chmod +x scripts/phase1-runtimes.sh
./scripts/phase1-runtimes.sh
```

The runtime launcher starts Phase1 with:

```bash
PHASE1_SAFE_MODE=0
PHASE1_ALLOW_HOST_TOOLS=1
PHASE1_BLEEDING_EDGE=1
```

Use it when you want Python, Rust/lang, C, plugins, or other guarded host runtime tools to work without manually toggling boot options.

## First commands to try

Inside Phase1:

```text
help
version --compare
roadmap
sysinfo
security
opslog status
theme list
theme linux status
dash
ls /
avim hello.py
lang support
update protocol
```

To run Python or other local language runtimes inside Phase1, use the runtime launcher above or start from boot with host trust enabled:

```text
4    # SHIELD off
t    # TRUST HOST on
1    # BOOT
```

Then:

```text
py hello.py
lang run python hello.py
lang run rust main.rs
```

## Major features

### Advanced Operator Console UI

- Neo Tokyo / futuristic terminal interface
- Minimal neon banner in `assets/phase1-banner.svg`
- Rainbow app icon/logo in `assets/phase1-rainbow-logo.svg`
- Static boot timestamp to avoid redraw glitches on older/narrow terminals
- Host-aware boot defaults for laptop, desktop, mobile, ASCII, and color terminals
- Laptop and desktop UI modes
- Boot configuration menu with clear SHIELD, TRUST HOST, EDGE, VAULT, device, storage, and runtime-related controls
- Command-aware HUD and AVIM mode HUD
- Linux color pack support for truecolor, 256-color, ANSI, and mono fallback

### Secure-by-default boot model

Phase1 starts with safe mode on and host tools disabled.

Host-backed commands are guarded by two gates:

1. **SHIELD off**: safe mode disabled
2. **TRUST HOST on**: explicit permission to run host tools

This prevents accidental execution of Python, compilers, plugins, updater execution, browser fetches, and other host-backed paths.

Fast host-runtime path:

```bash
./scripts/phase1-runtimes.sh
```

or from the boot selector:

```text
4
t
1
```

Host network mutation remains separately guarded with:

```bash
PHASE1_ALLOW_HOST_NETWORK_CHANGES=1
```

### Virtual OS simulation

Phase1 includes:

- Virtual filesystem with `/home`, `/proc`, `/dev`, `/tmp`, `/var/log`, `/etc`, and `/bin`
- Optional `/home` persistence through `phase1.state`
- Simulated process table and scheduler
- Background process commands
- Syscall-style read/write/spawn/kill boundaries
- Audit log exposed through `audit`
- PCIe / CR3 / CR4 / PCIDE educational hardware commands
- Privacy-safe simulated accounts model

### Shell and developer tools

- Registry-backed `help`, `man`, `complete`, aliases, and capability metadata
- Tab completion for commands and common arguments
- Up-arrow history recall and raw input editing
- Structured command chains with `;`, `&&`, and `||`
- Structured text pipelines
- Persistent shell history with secret redaction
- `avim` modal VFS editor
- `lang` runtime manager
- `update` and `update test` workflows

### Local operations logging

Phase1 writes local operations to:

```text
phase1.log
```

Useful commands:

```text
opslog status
opslog tail
opslog path
opslog clear
```

The logger is intended to help diagnose boot issues, terminal glitches, crashes, policy denials, and local operations without collecting private credentials.

### Storage / Git / Rust helper

The boot dock includes a guarded storage helper for project workspace checks, Git-oriented workflows, Rust support, and the language roadmap.

Useful examples:

```text
storage helper via d
lang support
lang security
update protocol
update test quick
update test full
```

The storage helper and host-backed Git/Rust operations stay guarded by the same SHIELD/TRUST HOST model.

### Phase1 Arena

Phase1 includes a clean-room text arena/game workspace:

```text
game workspace
arena start
```

Validation includes focused game integration tests.

### WASI-lite plugins

Phase1 includes WASI-lite plugin/runtime support designed around a Phase1 sandbox rather than direct host shell access by default.

Useful commands:

```text
wasm list
wasm inspect
wasm run
```

## Boot configuration

On launch, Phase1 opens a boot configuration screen.

Common controls:

```text
1 / Enter   boot into the shell
2           toggle color / neon output
3           toggle ASCII-compatible display
4           toggle SHIELD / safe mode
5           toggle quick boot
6           cycle device mode
l           laptop UI
w           desktop UI
t           TRUST HOST toggle
e           EDGE / bleeding-edge channel toggle
p           VAULT / persistent state toggle
d           storage / Git / Rust dock
7           reboot selector
8           shutdown / abort boot
9           save phase1.conf
0           reset saved config
h           help
```

Persistent state saves `/home` VFS files to `phase1.state`. Persistent command history writes sanitized entries to `phase1.history`. Do not store secrets in persistent state.

Environment defaults:

```bash
PHASE1_ASCII=1 cargo run
PHASE1_NO_COLOR=1 cargo run
PHASE1_SAFE_MODE=1 cargo run
PHASE1_SAFE_MODE=0 cargo run
PHASE1_SAFE_MODE=0 PHASE1_ALLOW_HOST_TOOLS=1 cargo run
PHASE1_ALLOW_HOST_NETWORK_CHANGES=1 cargo run
PHASE1_QUICK_BOOT=1 cargo run
PHASE1_DEVICE_MODE=laptop cargo run
PHASE1_DEVICE_MODE=desktop cargo run
PHASE1_MOBILE_MODE=1 cargo run
PHASE1_PERSISTENT_STATE=1 cargo run
PHASE1_HISTORY=off cargo run
PHASE1_THEME=matrix cargo run
```

## Command examples

Filesystem and text:

```text
pwd
ls /
mkdir lab
cd lab
echo hello world > note.txt
cat note.txt
cat note.txt | grep hello | wc -l
find /home -type f | sort
```

System and security:

```text
sysinfo
security
capabilities
audit
accounts
history status
bootcfg show
opslog status
```

Processes and hardware simulation:

```text
ps
spawn worker --background
jobs
kill 2
cr3
loadcr3 0x2000
pcide on
cr4
lspci
pcie
```

Networking, browser, and host-backed tools:

```text
ifconfig
iwconfig
wifi-scan
ping example.com
browser https://example.com
py hello.py
lang run python hello.py
```

The host-backed commands above require SHIELD off and TRUST HOST on.

Update and validation:

```text
version --compare
roadmap
update
update protocol
update latest --trust-host --check
update latest --trust-host --execute --build
update now --trust-host
update test quick
update test full
update test quick --trust-host --execute
```

AVIM editor:

```text
avim hello.py
```

Inside AVIM:

```text
i        enter INSERT mode
Esc      return to NORMAL mode
:wq      save and quit
:q!      quit without saving
:help    show editor help
```

## Structured pipelines

Phase1 supports lightweight structured shell pipelines:

```text
cat log.txt | grep alpha | wc -l
history | tail -5
ps | grep worker
find /home -type f | sort | uniq
version --compare | grep bleeding
```

Supported producers include `cat`, `echo`, `history`, `ps`, `ls`, `find`, `audit`, `env`, `version`, and `sysinfo`.

Supported filters include `grep`, `wc`, `head`, `tail`, `sort`, `uniq`, and `cut`.

## Themes and Linux color pack

Theme commands:

```text
theme list
theme status
theme matrix
theme cyber
theme amber
theme ice
theme synthwave
theme crimson
theme bleeding-edge
theme linux status
theme linux truecolor
theme linux 256
theme linux ansi
theme linux mono
theme linux preview
```

Linux hosts can use truecolor or fallback color modes to better match the Phase1 UI across terminals.

## Update protocol

The canonical protocol is documented in:

```text
UPDATE_PROTOCOL.md
```

The in-app reference is:

```text
update protocol
```

Safety rules:

- `update` defaults to a dry-run plan
- `--execute` is required for file mutation
- host update checks/execution require SHIELD off, TRUST HOST on, and `--trust-host`
- tracked local changes block execution instead of being overwritten
- updater output is sanitized before display

## Development checks

Recommended before every push:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
```

Optional deeper checks:

```bash
cargo clippy --all-targets -- -D warnings
cargo test --test smoke -- --nocapture
cargo test --test bleeding -- --nocapture
cargo test --test game -- --nocapture
```

Current validation coverage includes unit tests, storage binary tests, bleeding-edge behavior tests, game tests, and end-to-end smoke tests.

## Release workflow

After tests pass:

```bash
git status
git log -1 --oneline
git tag v3.10.7
git push origin v3.10.7
```

For the previous release line:

```text
RELEASE_NOTES_3.10.6.md
```

## Safety and privacy

Phase1 is an educational simulator. It should never need your GitHub password, personal access token, SSH private key, browser cookies, Apple ID, email password, or recovery codes.

Host-backed commands are explicit and guarded. Runtime state files such as `phase1.state`, `phase1.history`, and `phase1.log` are local operational artifacts. Do not publish them if they include sensitive local activity.

See:

```text
SECURITY.md
SECURITY_REVIEW.md
```

## Project assets

Brand banner:

```text
assets/phase1-banner.svg
```

App icon / logo:

```text
assets/phase1-rainbow-logo.svg
```

Splash / UI assets may also live in `assets/`.

## License

GPL-3.0
