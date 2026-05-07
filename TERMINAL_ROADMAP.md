# Phase1 Terminal Roadmap

Phase1 Terminal is the dedicated Linux/macOS launcher, profile, config, color, quality, and future terminal experience for Phase1.

The implementation has moved beyond a basic launcher. Phase1 Terminal now installs `phase1-terminal`, applies safe defaults, discovers a Phase1 checkout or binary, launches Phase1 through the best available path, manages allowlisted config, provides profiles, detects color capabilities, previews palettes, exposes doctor JSON, supports dry-run install/uninstall, runs self-tests and benchmark smoke tests, and adds Gina-aware launch shortcuts. This roadmap defines the remaining path from management layer to a more complete Phase1-native terminal experience.

## Guiding principles

- Keep Phase1 safe by default.
- Preserve Linux and macOS support equally.
- Make Phase1 beautiful where color is available and readable where mono output is preferred.
- Do not enable host trust, host networking, or host mutation from the terminal launcher.
- Prefer deterministic, testable shell scripts before larger native rewrites.
- Make every installer reversible and inspectable.
- Avoid clobbering user terminal settings.
- Keep `phase1-terminal` as the primary command; only create `terminal` as a safe optional alias.

## Current baseline

Implemented in the current Phase1 Terminal PR:

- `terminal/bin/phase1-terminal` launcher and management command.
- Generic installer: `scripts/install-phase1-terminal.sh`.
- Reversible uninstaller: `scripts/uninstall-phase1-terminal.sh`.
- Linux installer wrapper: `scripts/install-phase1-terminal-linux.sh`.
- Linux `.desktop` launcher support.
- macOS installer wrapper: `scripts/install-phase1-terminal-macos.sh`.
- macOS clickable `.command` launcher.
- Optional Terminal.app profile file.
- Config file: `~/.config/phase1/terminal.env`.
- `phase1-terminal doctor`, `doctor --verbose`, and `doctor --json`.
- `phase1-terminal env`.
- `phase1-terminal version`.
- Allowlisted `phase1-terminal config show|set|reset`.
- Safe profiles through `phase1-terminal profile list|show|apply`.
- Advanced color commands: `colors detect`, `colors swatches`, `theme list`, `theme preview`.
- Phase1 color environment export: `PHASE1_THEME`, `PHASE1_COLOR_MODE`, `PHASE1_TERMINAL_DETECTED_COLOR_MODE`, `PHASE1_TERMINAL_BANNER`.
- Session modes: `run`, `safe`, `dev`, `demo`, `base1`.
- Gina/security shortcuts: `gina`, `security`.
- Management helpers: `build`, `check`, `logs`.
- Quality command: `selftest`.
- Performance command: `benchmark`.
- Installer dry-run mode.
- Uninstaller dry-run mode.
- CI validation through `scripts/test-phase1-terminal.sh`.
- Test plan: `TERMINAL_TEST_CASES.md`.

## Phase 1 — Installer hardening

Goal: make Phase1 Terminal installation reliable, reversible, and safe across common Linux/macOS environments.

Status: mostly implemented.

Implemented:

- `scripts/uninstall-phase1-terminal.sh`.
- `phase1-terminal doctor --verbose`.
- `phase1-terminal doctor --json`.
- basic `PHASE1_HOME` validation.
- installer dry-run mode.
- uninstaller dry-run mode.
- CI validation for dry-run install/uninstall.

Remaining work:

- Detect missing Rust/Cargo and provide OS-specific install guidance without mutating the system.
- Detect PATH problems and print shell-specific fixes for bash, zsh, fish, and POSIX sh.
- Add installer idempotency tests that use a temporary prefix.
- Add checksum/manifest output for installed files.

Acceptance checks:

```bash
sh scripts/test-phase1-terminal.sh
sh scripts/install-phase1-terminal.sh --dry-run
sh scripts/uninstall-phase1-terminal.sh --dry-run
phase1-terminal doctor --json
phase1-terminal env
```

## Phase 2 — Profile, color, and UX polish

Goal: make Phase1 Terminal feel like a dedicated Phase1 environment instead of a generic shell wrapper.

Status: mostly implemented.

Implemented:

- generated terminal title.
- profile system.
- color capability detection.
- mono fallback with `NO_COLOR=1` or `PHASE1_COLOR_MODE=mono`.
- theme previews and swatches.
- color/theme environment export into Phase1.
- config presets through profile application:
  - `cyber`
  - `matrix`
  - `amber`
  - `mono`
  - `safe`
  - `developer`
  - `base1`
  - `ice`
  - `crimson`
- `phase1-terminal config show`.
- `phase1-terminal config set KEY=VALUE` with safe allowlisted keys.
- `phase1-terminal config reset`.

Remaining work:

- Add Linux desktop icon once a stable icon asset is selected.
- Add macOS profile import instructions with screenshots or text walkthrough.
- Add shell completion files for bash/zsh/fish.
- Add first-run banner rendering inside the launcher.
- Add profile-specific docs examples.

Acceptance checks:

```bash
phase1-terminal colors detect
phase1-terminal theme preview all
NO_COLOR=1 phase1-terminal theme preview all
phase1-terminal config show
phase1-terminal config set PHASE1_THEME=cyber
phase1-terminal config reset
phase1-terminal profile list
phase1-terminal profile apply matrix
phase1-terminal doctor
```

## Phase 3 — Native Phase1 session management

Goal: give Phase1 Terminal consistent launch modes for operators, developers, Base1 users, and demos.

Status: mostly implemented.

Implemented:

- `phase1-terminal run`.
- `phase1-terminal demo`.
- `phase1-terminal dev`.
- `phase1-terminal base1`.
- `phase1-terminal safe`.
- `phase1-terminal build` wrapper for `cargo build --release`.
- `phase1-terminal check` wrapper for validation commands.
- `phase1-terminal logs` for local Phase1 runtime files.
- graceful errors when no Phase1 home/binary and no Cargo are available.
- support for source, release binary, debug binary, `bin/phase1`, local `./phase1`, and PATH binary discovery.

Remaining work:

- Add explicit `PHASE1_BINARY` config override.
- Add `phase1-terminal check --quick`.
- Add `phase1-terminal check --full`.
- Add redacted log output mode.

Acceptance checks:

```bash
phase1-terminal safe
phase1-terminal dev
phase1-terminal build
phase1-terminal check
phase1-terminal logs
```

## Phase 4 — Quality and performance layer

Goal: make Phase1 Terminal self-checking and performance-aware.

Status: partially implemented.

Implemented:

- `phase1-terminal selftest`.
- `phase1-terminal benchmark [iterations]`.
- CI validation for self-test and benchmark smoke.
- `TERMINAL_TEST_CASES.md` quality/performance test plan.

Remaining work:

- Add machine-readable benchmark JSON.
- Add startup-path timing for source, debug binary, release binary, and PATH binary modes.
- Add temporary-prefix install/uninstall tests.
- Add benchmark thresholds once real device baselines exist.

Acceptance checks:

```bash
phase1-terminal selftest
phase1-terminal benchmark 25
sh scripts/test-phase1-terminal.sh
```

## Phase 5 — Gina-aware terminal workflows

Goal: integrate Phase1 Terminal with Gina while preserving offline/sandboxed defaults.

Status: partially implemented.

Implemented:

- `phase1-terminal gina` launches Phase1 with Gina guidance.
- `phase1-terminal security` launches Phase1 with a security-first Gina hint.
- verbose doctor checks for Gina plugin presence.
- post-launch hint: `Inside Phase1, try: gina security`.
- documentation for using Gina from Phase1 Terminal.
- no Gina/provider-backed behavior is enabled by the terminal.

Remaining work:

- Add `phase1-terminal gina --status` that reports Gina availability without launching Phase1.
- Add `phase1-terminal gina --doctor`.
- Add terminal-to-Gina roadmap cross-reference output.
- Add tests that simulate Gina plugin presence with a temporary Phase1 home.

Acceptance checks:

```bash
phase1-terminal doctor --verbose
phase1-terminal gina
phase1-terminal security
```

## Phase 6 — Packaging

Goal: make Phase1 Terminal installable through common packaging flows without requiring users to manually copy scripts.

Status: planned.

Planned work:

- Add release tarball layout.
- Add `make install` equivalent or `cargo xtask terminal-package` once `xtask` is merged.
- Add Homebrew formula draft for macOS.
- Add Debian package draft for Linux.
- Add AppImage or portable Linux archive exploration.
- Add checksums for release artifacts.
- Document upgrade and rollback path.

Acceptance checks:

```bash
phase1-terminal --version
phase1-terminal doctor
sh scripts/uninstall-phase1-terminal.sh --dry-run
```

## Phase 7 — Native terminal application exploration

Goal: decide whether Phase1 needs a real terminal emulator or should remain a launcher/profile layer.

Status: research only.

Exploration topics:

- Rust-native TUI shell wrapper.
- Dedicated Phase1 pseudo-terminal session manager.
- GPU-accelerated desktop terminal emulator feasibility.
- macOS app bundle feasibility.
- Linux desktop app feasibility.
- Accessibility and reduced-motion requirements.
- Security implications of embedding PTY/session management.

Decision gate:

Only build a full native terminal emulator if it provides clear value beyond existing Linux/macOS terminals and does not weaken Phase1 safety guarantees.

## Testing strategy

Every terminal change should include at least one of:

- shell syntax validation
- installer dry-run validation
- uninstaller dry-run validation
- doctor output validation
- color detection validation
- theme preview validation
- config parser validation
- profile validation
- self-test validation
- benchmark smoke validation
- docs update
- CI workflow coverage

Core validation command:

```bash
sh scripts/test-phase1-terminal.sh
```

Future validation commands:

```bash
phase1-terminal doctor --json
phase1-terminal colors detect
phase1-terminal theme preview all
phase1-terminal selftest
phase1-terminal benchmark 25
```

## Security requirements

Phase1 Terminal must never silently enable:

- host trust
- host shell mutation
- host network mutation
- external AI provider calls
- credential storage
- browser cookie access
- unredacted history/log export

Phase1 Terminal may guide the user to enable higher-trust Phase1 modes, but those modes must stay explicit and policy-controlled inside Phase1.

## Success criteria

Phase1 Terminal is successful when:

- Linux and macOS users can install it with one command.
- `phase1-terminal doctor` clearly diagnoses environment problems.
- `phase1-terminal doctor --json` supports machine-readable checks.
- Phase1 receives color/theme settings reliably.
- theme previews are beautiful on color terminals and readable in mono fallback.
- the launcher starts Phase1 consistently from source or binary installs.
- safe profiles are easy to apply.
- safe defaults are preserved.
- Gina security guidance is discoverable.
- uninstall/upgrade paths are documented and tested.
- CI catches script, color, asset, and documentation regressions.
