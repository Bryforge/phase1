# Phase1 Terminal Roadmap

Phase1 Terminal is the dedicated Linux/macOS launcher, profile, and future terminal experience for Phase1.

The initial implementation is intentionally conservative: it installs `phase1-terminal`, applies safe Phase1 defaults, discovers a Phase1 checkout or binary, and launches Phase1 through the best available path. This roadmap defines the path from launcher/profile layer to a more complete Phase1-native terminal experience.

## Guiding principles

- Keep Phase1 safe by default.
- Preserve Linux and macOS support equally.
- Do not enable host trust, host networking, or host mutation from the terminal launcher.
- Prefer deterministic, testable shell scripts before larger native rewrites.
- Make every installer reversible and inspectable.
- Avoid clobbering user terminal settings.
- Keep `phase1-terminal` as the primary command; only create `terminal` as a safe optional alias.

## Current baseline

Implemented in the first Phase1 Terminal PR:

- `terminal/bin/phase1-terminal` launcher.
- Generic installer: `scripts/install-phase1-terminal.sh`.
- Linux installer wrapper: `scripts/install-phase1-terminal-linux.sh`.
- Linux `.desktop` launcher support.
- macOS installer wrapper: `scripts/install-phase1-terminal-macos.sh`.
- macOS clickable `.command` launcher.
- Optional Terminal.app profile file.
- Config file: `~/.config/phase1/terminal.env`.
- Diagnostic command: `phase1-terminal doctor`.
- Environment dump command: `phase1-terminal env`.
- CI syntax/asset validation through `scripts/test-phase1-terminal.sh`.

## Phase 1 — Installer hardening

Goal: make Phase1 Terminal installation reliable, reversible, and safe across common Linux/macOS environments.

Planned work:

- Add `scripts/uninstall-phase1-terminal.sh`.
- Add `phase1-terminal doctor --verbose`.
- Add `phase1-terminal doctor --json` for machine-readable checks.
- Validate `PHASE1_HOME` points to a real Phase1 checkout or install root.
- Detect missing Rust/Cargo and provide OS-specific install guidance without mutating the system.
- Detect PATH problems and print shell-specific fixes for bash, zsh, fish, and POSIX sh.
- Add installer dry-run mode.
- Add installer idempotency tests.

Acceptance checks:

```bash
sh scripts/test-phase1-terminal.sh
sh scripts/install-phase1-terminal.sh --dry-run
phase1-terminal doctor
phase1-terminal env
```

## Phase 2 — Profile and UX polish

Goal: make Phase1 Terminal feel like a dedicated Phase1 environment instead of a generic shell wrapper.

Planned work:

- Add generated terminal title and session banner.
- Add config presets:
  - `cyber`
  - `matrix`
  - `amber`
  - `mono`
  - `safe`
  - `developer`
- Add `phase1-terminal config show`.
- Add `phase1-terminal config set KEY=VALUE` with safe allowlisted keys.
- Add `phase1-terminal config reset`.
- Add Linux desktop icon once a stable icon asset is selected.
- Add macOS profile import instructions with screenshots or text walkthrough.
- Add shell completion files for bash/zsh/fish.

Acceptance checks:

```bash
phase1-terminal config show
phase1-terminal config set PHASE1_THEME=cyber
phase1-terminal config reset
phase1-terminal doctor
```

## Phase 3 — Native Phase1 session management

Goal: give Phase1 Terminal consistent launch modes for operators, developers, Base1 users, and demos.

Planned work:

- Add launch profiles:
  - `phase1-terminal run`
  - `phase1-terminal demo`
  - `phase1-terminal dev`
  - `phase1-terminal base1`
  - `phase1-terminal safe`
- Add `phase1-terminal build` wrapper for `cargo build --release`.
- Add `phase1-terminal check` wrapper for Phase1 validation commands.
- Add `phase1-terminal logs` for local Phase1 runtime logs.
- Add graceful error messages when no binary and no Cargo are available.
- Add support for launching a specific Phase1 binary path.

Acceptance checks:

```bash
phase1-terminal safe
phase1-terminal dev
phase1-terminal build
phase1-terminal check
phase1-terminal logs
```

## Phase 4 — Gina-aware terminal workflows

Goal: integrate Phase1 Terminal with Gina while preserving offline/sandboxed defaults.

Planned work:

- Add `phase1-terminal gina` to launch Phase1 directly into Gina guidance.
- Add `phase1-terminal security` to launch Phase1 with a security-first command hint.
- Add terminal doctor checks for Gina plugin presence.
- Add optional post-launch hint: `Inside Phase1, try: gina security`.
- Add documentation for using Gina from Phase1 Terminal.
- Keep all Gina/provider-backed behavior policy-gated and disabled by default.

Acceptance checks:

```bash
phase1-terminal doctor
phase1-terminal gina
phase1-terminal security
```

## Phase 5 — Packaging

Goal: make Phase1 Terminal installable through common packaging flows without requiring users to manually copy scripts.

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
phase1-terminal uninstall --dry-run
```

## Phase 6 — Native terminal application exploration

Goal: decide whether Phase1 needs a real terminal emulator or should remain a launcher/profile layer.

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
- doctor output validation
- config parser validation
- docs update
- CI workflow coverage

Core validation command:

```bash
sh scripts/test-phase1-terminal.sh
```

Future validation commands:

```bash
phase1-terminal doctor --json
phase1-terminal config show
phase1-terminal uninstall --dry-run
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
- the launcher starts Phase1 consistently from source or binary installs.
- safe defaults are preserved.
- Gina security guidance is discoverable.
- uninstall/upgrade paths are documented and tested.
- CI catches script, asset, and documentation regressions.
