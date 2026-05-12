# phase1 system test report

Date: 2026-05-05
Release under test: phase1 v3.5.0

## Scope

This pass reviewed the command surface and added an end-to-end smoke test suite that drives the compiled `phase1` binary through stdin the same way a user does from a terminal.

The new test file is:

```text
tests/smoke.rs
```

## Covered command groups

The smoke suite covers:

- boot screen startup
- `help`
- `complete`
- `man`
- `version`
- `pwd`
- `ls`
- `mkdir`
- `cd`
- `echo` redirection
- `cat`
- `cp`
- `mv`
- `rm`
- `/proc/version`
- `/proc/cpuinfo`
- `ps`
- `spawn`
- `jobs`
- `cr3`
- `loadcr3`
- `pcide`
- `cr4`
- `lspci`
- `pcie`
- `audit`
- `whoami`
- `id`
- `export`
- environment expansion with `$VAR`
- `unset`
- `env`
- `browser phase1`
- `sandbox`
- `date`
- `uptime`
- `hostname`
- expected error messages for bad paths, bad CR3 values, missing WiFi arguments, and unknown commands

## Fixes made during this pass

### Restored the `date` command

`date` called a missing `now_unix()` helper. Added the helper back so the command compiles and returns a UNIX timestamp.

### Removed a compiler warning in network discovery

`src/network.rs` had an unnecessary `mut` around the macOS `ifconfig` command object. Removed it.

### Fixed built-in command precedence

Plugins were previously checked before built-ins. That meant a plugin named `ls.py`, `ps.py`, or similar could accidentally override a core command. Built-ins now resolve first, and plugins only run as fallback commands.

### Hardened Python command usage

`python -c` now requires inline code and prints usage when no code is provided.

### Improved clear behavior

`clear` now sends a terminal clear escape sequence instead of printing many blank lines.

## How to run locally

From the repo root:

```bash
git pull origin master
cargo fmt
cargo check
cargo test
cargo run
```

For the new end-to-end suite only:

```bash
cargo test --test smoke -- --nocapture
```

## Environment limitation

The ChatGPT execution container available for this pass does not have `rustc` or `cargo` installed, so the new smoke suite could not be executed inside this environment. The repository now contains the tests and fixes so they can run in the local Mac environment and in CI where Rust is available.
