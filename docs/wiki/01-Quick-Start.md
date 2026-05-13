# Quick Start

![Edge](https://img.shields.io/badge/edge-v6.0.0-00d8ff) ![Stable](https://img.shields.io/badge/stable-v5.0.0-39ff88) ![Safe Mode](https://img.shields.io/badge/safe%20mode-default%20on-39ff88)

This page gets a new user from clone to first commands without weakening Phase1's default safety posture.

## Requirements

- Git
- A terminal that supports ANSI output
- Rust toolchain with Cargo for native builds and tests
- macOS, Linux, or another environment capable of running Rust CLI tools

## Install and run

Fresh clone, simplest launch:

> [!TIP]
> TRY THIS
>
> ```bash
> git clone https://github.com/Bryforge/phase1.git
> cd phase1
> sh phase1
> ```

After the launcher is executable, this also works:

```bash
./phase1
```

Rust-native launch remains available:

```bash
cargo run
```

At the boot selector, press `1` or `Enter` to enter the shell.

## Optional local terminal command

Install a local `phase1` command on macOS/Linux:

```bash
sh scripts/install-phase1-command.sh
phase1
```

## First host checks

Run these from the repository root:

```bash
sh phase1 version
sh phase1 doctor
sh phase1 selftest
```

## First Phase1 session

> [!TIP]
> TRY THIS INSIDE PHASE1
>
> ```text
> help
> help ui
> help flows
> cat readme.txt
> wiki
> wiki-quick
> version
> version --compare
> security
> capabilities
> sysinfo
> roadmap
> ```

You should learn:

- which Phase1 version is running
- whether safe mode is active
- whether host trust is enabled
- where the built-in guide and wiki live
- which commands require host access

## Create and edit a file

```text
ned notes.txt
```

Or use AVIM:

```text
avim hello.fyr
```

Inside AVIM:

```text
i                                      enter INSERT mode
fn main() -> i32 { print("hi"); return 0; }
Esc                                    return to NORMAL mode
:wq                                    save and quit
```

Then run it:

```text
fyr run hello.fyr
```

Expected output:

```text
hi
```

## Build a release binary

```bash
cargo build --release
./target/release/phase1
```

## Run validation

Run the quick repository gate before ordinary pushes:

```bash
sh scripts/quality-check.sh quick
```

Run Rust-specific validation when changing source code:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Run focused documentation or Base1 gates when changing those areas:

```bash
sh scripts/quality-check.sh base1-docs
sh scripts/quality-check.sh security-crypto-docs
```

## Enable local language runtimes

Phase1 blocks host-backed tools by default. Python, Rust host execution, C, browser fetches, plugins, Git helpers, and host network inspection require explicit trust.

```bash
chmod +x scripts/phase1-runtimes.sh
./scripts/phase1-runtimes.sh
```

The launcher sets runtime-oriented host gates for local development. Only use it on a machine and repository state you trust.

## Manual boot equivalent

At the boot selector:

```text
4    SHIELD off
t    TRUST HOST on
1    BOOT
```

Then try:

```text
py hello.py
browser example.com
ping example.com
lang support
```

## Read the built-in guide

Every fresh boot creates:

```text
/home/readme.txt
```

Read it with:

```text
cat readme.txt
```

The in-system guide reflects the package version currently booted.

## Next pages

- Read [Version Guide](02-Version-Guide.md) to understand edge, stable, and compatibility labels.
- Read [Boot Modes and Security](03-Boot-Modes-and-Security.md) before enabling host-backed workflows.
- Read [Fyr Native Language](14-Fyr-Native-Language.md) for Phase1-native scripting.
- Read [Base1 OS Track](13-Base1-OS-Track.md) for boot, recovery, and hardware-validation planning.
