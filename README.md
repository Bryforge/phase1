# phase1

**phase1 v0.3.0** — Educational Embedded Operating System

A lightweight, terminal-based educational OS simulator written in **Rust**. It demonstrates core operating system concepts — process management, memory accounting, kernel simulation, and more — all running safely in userspace with optional sandboxing and full Python plugin extensibility.

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey)

## Features

### Simulated OS Components
- **Stateful process management** (`ps`, `kill` — processes can be created and terminated)
- **Dynamic memory accounting** (`free` / `mem`)
- **Real-time system information** (`uptime` using live `Instant`, `uname` / `kernel`)
- **Kernel simulation** with colored boot sequence and sandbox details

### Built-in Shell Commands
- `help` — Show all available commands
- `ls`, `pwd`, `cd`, `cat`, `echo`, `clear`
- `python` / `py` — Execute Python code inline or interactively
- `plugins` / `plugin` — List and manage Python plugins
- `ps` / `proc`, `free` / `mem`, `uptime`, `uname` / `kernel`, `kill <pid>`
- `sandbox` / `nsinfo` — Show isolation/sandbox information
- `exit` / `quit` / `shutdown`

### Extensibility & Plugins
- Drop a `.py` file into the `plugins/` directory → instantly becomes a new shell command
- The `plugins/` directory is **automatically created** on first run
- Plugins receive arguments and run via `python3`
- Example plugins (e.g., `hello.py`, `sysinfo.py`) can be added easily

### Isolation & Safety
- **Linux**: Full namespace sandboxing (user, mount, pid, network, uts)
- **macOS**: Reduced host privileges mode
- Zero new dependencies beyond Rust standard library

## Quick Start

### Prerequisites
- Rust (1.75 or newer) — [Install Rust](https://www.rust-lang.org/tools/install)

### 1. Clone & Build
```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo build --release
