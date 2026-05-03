# phase1

**phase1 v0.4.0** — Educational Embedded Operating System

A terminal-based educational OS simulator written in **Rust**. Demonstrates core OS concepts including process management, memory accounting, kernel simulation, **dynamic process spawning**, and **disk simulation** in a safe userspace environment with Python plugin support.

<image-card alt="Rust" src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust" ></image-card>
<image-card alt="License" src="https://img.shields.io/badge/License-GPLv3-blue.svg" ></image-card>
<image-card alt="Platform" src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey" ></image-card>

## Features

### Simulated OS Components
- Process table (`ps`, `kill`, **new: `spawn`**, **new: `top`**)
- Memory accounting (`free`)
- **New:** Disk accounting (`df`)
- System information (`uptime`, `uname`)
- Kernel simulation

### Shell Commands (new in v0.4)
- `spawn <name>` — Dynamically create new simulated processes
- `df` — Simulated disk usage
- `whoami` / `id` — User identity
- `top` — Enhanced process monitor with fake CPU% visualization

### Shell Commands (existing)
- `help`, `ls`, `pwd`, `cd`, `cat`, `echo`, `clear`, `exit`
- `python` / `py` — Execute Python code directly
- `plugins` — List/extend with Python plugins

### Extensibility
Place `.py` files in the `plugins/` directory to register new commands instantly.

### Isolation
- On Linux: Full namespace sandboxing
- On macOS: Reduced host privileges

## Quick Start

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo build --release
./target/release/phase1
