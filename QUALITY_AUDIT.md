# phase1 quality audit

Date: 2026-05-06

## Scope

This pass focused on repository quality gates, runtime smoke coverage, storage/Git/Rust helper confidence, mobile terminal usability, safety, and automated validation for the Rust terminal OS simulator.

## Changes made

- Added a dedicated `src/ui.rs` module for terminal UI output.
- Replaced the wide boot output with a compact mobile-first operator console card.
- Reworked the interactive prompt to use a shorter path display, including `~` for `/home`.
- Added unit tests for prompt path compaction and UI box sizing.
- Added GitHub Actions CI in `.github/workflows/rust-ci.yml`.
- Updated CI so Rust validation runs on pull requests, `master`, `main`, `release/**`, and manual dispatch.
- CI validates formatting, compilation, linting, unit tests, integration tests, RustSec audit, and dependency policy.
- Added scripted integration smoke coverage for the main `phase1` shell in `tests/smoke.rs`.
- Added dedicated storage helper smoke coverage in `tests/storage_smoke.rs`.

## Automated validation plan

The CI gate intentionally uses the standard Rust toolchain commands:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Rationale:

- `cargo fmt --all -- --check` enforces consistent Rust formatting.
- `cargo check --all-targets` checks every target quickly without final code generation.
- `cargo clippy --all-targets -- -D warnings` promotes lint findings to failures.
- `cargo test --all-targets` runs unit tests plus the smoke integration suites.
- `cargo audit` checks RustSec advisories.
- `cargo deny check` enforces dependency policy.

## Smoke-test coverage

The main shell smoke suite exercises:

- boot flow and safe defaults
- command map, manual pages, completions, and version output
- VFS read/write/copy/move/delete paths
- command chaining with `;`, `&&`, and `||`
- text tools such as `grep`, `wc`, `head`, `tail`, and `find`
- `/proc`, process, audit, scheduler, CR3/CR4, and PCIe commands
- network commands in safe mode
- browser, Python, and host-tool denial behavior
- persistent `/home` state restore
- persistent history sanitization
- dashboard, aliases, capabilities, and privacy-safe account reports

The storage helper smoke suite exercises:

- read-only `storage status` in guarded mode
- blocking of mutating storage, Git, and Rust actions by default
- explicit trusted storage initialization
- guarded Rust project creation under `phase1.workspace`
- repository/project name validation
- unsafe Git URL rejection
- major language roadmap coverage

## Static review findings

### UI / terminal output

The current UI is significantly stronger than the early wide banner design. The boot card now adapts to mobile/laptop/desktop modes, supports ASCII/no-color fallback, and preserves a recognizable Neo Tokyo operator-console identity without requiring graphical dependencies.

### Safety

The current safety posture is strong for an educational userspace simulator. Safe mode defaults to on, host tools require explicit opt-in, browser schemes are limited, host command paths use timeouts, WiFi mutation is dry-run unless explicitly enabled, persistent state is limited to Phase1-managed `/home`, and common runtime/credential files are ignored by Git.

### Maintainability

The project has useful modules for UI, networking, browser, updater, storage helper, registry, and shell behavior. The largest remaining maintainability risk is duplicated host-command execution policy across browser, network, updater, Python/C runtime, plugin, and storage helper paths. A future `host_tools` module should centralize timeout, redaction, stdin, audit, and policy checks.

### Testing

The repository now includes broad unit coverage plus runtime smoke coverage. The smoke suites run through Cargo integration tests and should execute in CI on PRs and default-branch pushes.

## Recommended next pass

1. Add branch protection in GitHub settings so `Rust CI / fmt-check-clippy-test-security` is required before merging into `master`.
2. Split host-tool execution into a single utility module shared by browser, network, Python/C, plugins, updater, and storage helper code.
3. Add a small release script that runs the CI command set locally before tagging.
4. Add artifact validation for bundled `.wasm` / `.wasi` plugin fixtures.
5. Add coverage for failure cases around malformed persisted state and corrupted history files.
