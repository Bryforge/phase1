# phase1

**phase1** is a terminal-based educational operating-system simulator written in Rust.

It demonstrates OS concepts in safe userspace: a virtual filesystem, a simulated process table and scheduler, shell commands, `/proc`-style files, PCIe enumeration, plugins, simple editors, host-tool integration, and a terminal browser.

## What changed in this improvement pass

- Reformatted the crate into readable Rust source instead of dense single-line files.
- Removed the external `chrono` dependency; the simulator now builds with only the Rust standard library.
- Added quote-aware command parsing, safer environment expansion, and deterministic sorted `ls` output.
- Made `kill`, `nice`, `jobs`, `fg`, and `bg` mutate simulated process state instead of only printing success.
- Hardened host integrations:
  - `browser` allows only `http://` and `https://`, rejects `file://`, and bounds curl time/size.
  - `ping`, Python plugins, Python execution, and C compilation use timeouts.
  - `wifi-connect` is dry-run by default. Set `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` to allow real host network changes.
- Replaced fake Linux network interface data with best-effort host discovery and honest fallbacks.
- Added clearer man pages and a code-review report.

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
man browser
ls -l /
spawn worker --background
jobs
nice 1002 -5
kill 1002
echo "hello world" > note.txt
cat note.txt
browser https://example.com
```

## Safety model

phase1 is an educational simulator. Its VFS, processes, and scheduler are simulated in memory. Some features integrate with host tools (`python3`, `gcc`/`clang`, `curl`, `ping`, `nmcli`, `networksetup`). Those integrations are guarded with argument validation, timeouts, and dry-run defaults where the command could mutate host state.
