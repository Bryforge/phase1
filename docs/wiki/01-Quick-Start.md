# Quick Start

![Edge](https://img.shields.io/badge/edge-v3.10.9--dev-00d8ff) ![Stable](https://img.shields.io/badge/stable-v3.10.7-39ff88)

This page gets a new user from clone to first commands.

## Requirements

- Rust toolchain with Cargo
- Git
- A terminal that supports ANSI output

## Install and run

> [!TIP]
> TRY THIS
>
> ```bash
> git clone https://github.com/Bryforge/phase1.git
> cd phase1
> cargo run
> ```

At the boot screen, press `1` or `Enter` to enter the shell.

## First session

> [!TIP]
> TRY THIS INSIDE PHASE1
>
> ```text
> help
> cat readme.txt
> version
> version --compare
> security
> sysinfo
> roadmap
> ```

## Build a release binary

> [!TIP]
> TRY THIS
>
> ```bash
> cargo build --release
> ./target/release/phase1
> ```

## Run validation

> [!TIP]
> TRY THIS BEFORE PUSHING
>
> ```bash
> cargo fmt --all -- --check
> cargo check --all-targets
> cargo test --all-targets
> ```

## Enable local language runtimes

Phase1 blocks host-backed tools by default. To run Python, Rust, C, browser fetches, plugins, or host network inspection, use the runtime launcher:

> [!TIP]
> TRY THIS
>
> ```bash
> chmod +x scripts/phase1-runtimes.sh
> ./scripts/phase1-runtimes.sh
> ```

The launcher sets:

```bash
PHASE1_SAFE_MODE=0
PHASE1_ALLOW_HOST_TOOLS=1
PHASE1_BLEEDING_EDGE=1
```

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

## Create and edit a file

> [!TIP]
> TRY THIS
>
> ```text
> avim hello.py
> ```

Inside AVIM:

```text
i                 enter INSERT mode
print("hello")    type a line
Esc               return to NORMAL mode
:wq               save and quit
```

Then run it when host runtimes are enabled:

```text
py hello.py
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

The in-system guide reflects the exact package version currently booted.
