# phase1

**Terminal-based educational OS simulator in Rust.**

Research project demonstrating core operating system concepts in safe userspace.

## Features

- In-memory VFS with permissions, `/proc`, `/dev`, and dynamic entries
- Preemptive priority scheduler with jobs (`jobs`, `fg`, `bg`)
- Process management: `ps`, `top`, `spawn`, `kill`, `nice`
- File ops: `ls`, `cd`, `pwd`, `cat`, `mkdir`, `touch`, `rm`, `cp`, `mv`, `tree`
- System commands: `free`, `df`, `uname`, `uptime`, `date`, `whoami`, `id`, `dmesg`, `ifconfig`, `ping`, …
- Environment variables, history, redirection
- Python plugin system (`plugins/`)
- Linux namespace sandboxing (reduced privileges on macOS)

## Quick Start

```bash
git clone https://github.com/Bryforge/phase1.git
cd phase1
cargo build --release
./target/release/phase1
