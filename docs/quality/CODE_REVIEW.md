# phase1 code review and improvement plan

Reviewed repository: `https://github.com/Bryforge/phase1`

## Executive summary

phase1 is a solid educational idea: a terminal OS simulator in Rust with a VFS, simulated scheduler, plugins, networking, browser, editor, and C/Python integrations. The current code is functional-looking but should be improved before it is treated as a reliable teaching tool or distributed binary.

The most important changes in this pack are:

1. Make the source maintainable by restoring normal Rust formatting and separating concerns.
2. Prevent host-tool integrations from surprising users or exposing local data.
3. Make simulated process commands actually update simulated process state.
4. Stop reporting fake network data as if it were real.
5. Align versions and documentation.
6. Improve shell parsing and VFS correctness.

## High-priority findings

### 1. Version drift

The package metadata says `3.0.0`, the README and browser text refer to `3.0.0`, but the shell prints `3.3.1`.

Fix: define one `VERSION` constant in `kernel.rs` and use it everywhere.

### 2. Unsafe / surprising browser behavior

`browser` shells out to `curl` on arbitrary user input. Because it does not restrict schemes, a command like `browser file:///etc/passwd` may read local host files if curl supports that scheme.

Fix: allow only `http://` and `https://`; use curl `--proto =http,https`, `--max-time`, `--max-filesize`, `--fail`, and a process timeout.

### 3. HTML stripping can panic or corrupt Unicode

The old stripper lowercases the entire page and then indexes it using character enumeration offsets. Character indexes are not byte indexes in Rust strings, so non-ASCII content can panic.

Fix: use a char-state parser that never slices strings at incorrect byte offsets.

### 4. Process commands claim success but do not mutate state

`kill` and `nice` return success strings but do not modify process records. This makes scheduler demos misleading.

Fix: implement real simulated state changes for `kill`, `nice`, `jobs`, `fg`, and `bg`.

### 5. Fake network data

Linux refresh ignores actual `ip addr` output and hardcodes `eth0`, `wlan0`, private IPs, and fake MAC addresses.

Fix: parse `ip -o -4 addr show` and `/sys/class/net/<iface>/address` where available. Fall back honestly to loopback.

### 6. Host network mutation

`wifi-connect` can call `networksetup` or `nmcli` and mutate the real host network.

Fix: make it dry-run by default. Require `PHASE1_ALLOW_HOST_NETWORK_CHANGES=1` for real changes.

### 7. Shell parsing is whitespace-only

The old main loop uses `split_whitespace`, so quotes and redirection are brittle.

Fix: add a small quote-aware tokenizer that handles `'...'`, `"..."`, `>`, `>>`, and escapes.

### 8. Temp file collisions and host execution timeouts

C and Python commands use predictable temp filenames or no timeout. That can collide across sessions and can hang the shell.

Fix: use unique filenames and enforce timeouts for compilation/execution.

## Files changed

- `Cargo.toml`
- `.gitignore`
- `README.md`
- `src/main.rs`
- `src/kernel.rs`
- `src/commands.rs`
- `src/browser.rs`
- `src/network.rs`
- `src/ned.rs`
- `src/man.rs`
- `plugins/hello.py`
- `plugins/demo.py`

## Validation note

I could not run `cargo check` in this environment because the Rust toolchain was not available. The replacement code avoids external dependencies and is written to be checked with:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```
