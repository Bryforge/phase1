# phase1

**phase1 v0.2** — Educational Embedded Operating System

A terminal-based educational OS simulator written in Rust. Demonstrates core OS concepts including process management, memory accounting, and kernel simulation in a safe userspace environment with Python plugin support.

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey)

## Features

### Simulated OS Components
- Process table (`ps`, `kill`)
- Memory accounting (`free`)
- System information (`uptime`, `uname`)
- Kernel simulation

### Shell Commands
- `help`, `ls`, `pwd`, `cd`, `cat`, `echo`, `clear`, `exit`
- `python` / `py` — Execute Python code directly
- `plugins` — List available Python extensions

### Extensibility
Place `.py` files in the `plugins/` directory to register new commands instantly. Examples included: `hello`, `sysinfo`.

### Isolation
- On Linux: Full namespace sandboxing (user, mount, pid, network, uts)
- On macOS: Runs with reduced host privileges

## Quick Start

```bash
git clone https://github.com/chasebryan/phase1.git
cd phase1
cargo build --release
./target/release/phase1
