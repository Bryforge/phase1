# phase1

phase1 v1.0.0 is a terminal-based educational OS simulator written in Rust. It is a research project demonstrating core operating system concepts in safe userspace.

## Features

- In-memory virtual filesystem (VFS) with permissions, /proc, /dev, and dynamic entries
- Preemptive process scheduler with priorities, jobs, and background tasks
- Process management (ps, top, spawn, kill, nice, jobs, fg, bg)
- Memory and disk accounting (free, df)
- File operations (ls, cd, pwd, cat, mkdir, touch, rm, cp, mv, tree)
- Environment variables with expansion, command history, and redirection
- System commands (uname, uptime, date, whoami, id, dmesg, vmstat, hostname, ifconfig, ping)
- Python plugin system (plugins/ directory)
- Linux namespace sandboxing (macOS: reduced privileges)

## Quick Start

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo build --release
./target/release/phase1
