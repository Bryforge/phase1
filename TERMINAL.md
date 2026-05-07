# Phase1 Terminal

Phase1 Terminal is the dedicated terminal launcher and profile layer for Phase1 on Linux and macOS.

The installed command is:

```bash
phase1-terminal
```

A shorter `terminal` alias can also be installed when safe.

Roadmap: [`TERMINAL_ROADMAP.md`](TERMINAL_ROADMAP.md). Test cases: [`TERMINAL_TEST_CASES.md`](TERMINAL_TEST_CASES.md).

## Name

The project name is **Phase1 Terminal**. The command is `phase1-terminal` because `terminal` can conflict with existing system tools, shell aliases, or user scripts. The installer can create `terminal` as an alias only when requested or when no existing command is found.

## What it does

Phase1 Terminal is a Phase1-specific terminal management layer that:

- loads Phase1 terminal configuration
- discovers a Phase1 checkout or binary
- prefers built binaries and falls back to `cargo run`
- detects terminal color capability
- exports Phase1 color/theme environment values
- previews palettes and theme swatches
- manages safe profiles and config presets
- provides diagnostics, self-tests, and benchmark smoke tests
- provides build/check/log helpers
- exposes Gina/security launch shortcuts
- installs Linux and macOS launch integrations
- supports dry-run install and reversible uninstall

## Install

Linux:

```bash
sh scripts/install-phase1-terminal-linux.sh
phase1-terminal doctor --verbose
phase1-terminal theme preview all
phase1-terminal
```

macOS:

```bash
sh scripts/install-phase1-terminal-macos.sh
phase1-terminal doctor --verbose
phase1-terminal theme preview all
phase1-terminal
```

Generic:

```bash
sh scripts/install-phase1-terminal.sh
sh scripts/install-phase1-terminal.sh --dry-run
```

## Uninstall

```bash
sh scripts/uninstall-phase1-terminal.sh
sh scripts/uninstall-phase1-terminal.sh --dry-run
sh scripts/uninstall-phase1-terminal.sh --remove-config
```

By default, uninstall keeps `~/.config/phase1/terminal.env` so user config is not deleted accidentally.

## Configuration

Default config file:

```text
~/.config/phase1/terminal.env
```

Example:

```sh
PHASE1_HOME="$HOME/phase1"
PHASE1_TERMINAL_TITLE="Phase1 Terminal"
PHASE1_TERMINAL_PROFILE="default"
PHASE1_THEME="cyber"
PHASE1_COLOR_MODE="auto"
PHASE1_TERMINAL_BANNER="1"
PHASE1_SAFE_MODE="1"
PHASE1_MOBILE_MODE="0"
PHASE1_DEVICE_MODE="desktop"
PHASE1_ASCII="0"
PHASE1_TERMINAL_HINTS="1"
PHASE1_TERMINAL_PERF_BUDGET_MS="500"
```

Config commands:

```bash
phase1-terminal config show
phase1-terminal config set PHASE1_THEME=matrix
phase1-terminal config set PHASE1_COLOR_MODE=truecolor
phase1-terminal config set PHASE1_HOME="$PWD"
phase1-terminal config reset
```

Only allowlisted config keys can be changed through `config set`.

## Color and themes

Phase1 Terminal detects terminal color support and exports both configured and detected color mode for Phase1.

Color modes:

```text
auto
truecolor
256
ansi
mono
```

Themes:

```text
cyber
matrix
amber
mono
safe
developer
base1
ice
synthwave
crimson
```

Color commands:

```bash
phase1-terminal colors detect
phase1-terminal colors swatches
phase1-terminal theme list
phase1-terminal theme preview cyber
phase1-terminal theme preview matrix
phase1-terminal theme preview all
NO_COLOR=1 phase1-terminal theme preview all
```

Phase1 receives:

```text
PHASE1_THEME
PHASE1_COLOR_MODE
PHASE1_TERMINAL_DETECTED_COLOR_MODE
PHASE1_TERMINAL_BANNER
```

This lets Phase1 be vivid where colors are available and readable where mono output is preferred.

## Profiles

```bash
phase1-terminal profile list
phase1-terminal profile show
phase1-terminal profile apply cyber
phase1-terminal profile apply matrix
phase1-terminal profile apply amber
phase1-terminal profile apply mono
phase1-terminal profile apply safe
phase1-terminal profile apply developer
phase1-terminal profile apply base1
phase1-terminal profile apply ice
phase1-terminal profile apply crimson
```

Profiles are safe by default. They change launch defaults such as theme, color mode, ASCII mode, and device mode.

## Commands

```bash
phase1-terminal                 # launch Phase1
phase1-terminal run             # launch Phase1
phase1-terminal safe            # launch with safe profile defaults
phase1-terminal dev             # launch with developer-friendly safe defaults
phase1-terminal demo            # launch with demo title/theme defaults
phase1-terminal base1           # launch with Base1-focused safe defaults
phase1-terminal gina            # launch Phase1 with Gina hint
phase1-terminal security        # launch Phase1 with Gina security hint
phase1-terminal build           # cargo build --release in Phase1 home
phase1-terminal check           # fmt/check/test in Phase1 home
phase1-terminal logs            # tail local Phase1 runtime files
phase1-terminal doctor          # check environment and discovery
phase1-terminal doctor --verbose
phase1-terminal doctor --json
phase1-terminal colors detect
phase1-terminal colors swatches
phase1-terminal theme list
phase1-terminal theme preview all
phase1-terminal selftest
phase1-terminal benchmark 25
phase1-terminal env
phase1-terminal version
phase1-terminal help
```

## Recommended build flow

```bash
cargo build --release
phase1-terminal
```

If no built binary exists, Phase1 Terminal falls back to:

```bash
cargo run --
```

## Gina workflow

```bash
phase1-terminal gina
phase1-terminal security
```

Inside Phase1, use:

```text
gina security
gina optimize
gina consistency
```

## Quality and performance tests

```bash
sh scripts/test-phase1-terminal.sh
phase1-terminal selftest
phase1-terminal benchmark 25
```

Full test-case guide: [`TERMINAL_TEST_CASES.md`](TERMINAL_TEST_CASES.md).

## Roadmap

The terminal roadmap covers installer hardening, profile UX, session management, Gina-aware workflows, packaging, and future native terminal exploration.

Read: [`TERMINAL_ROADMAP.md`](TERMINAL_ROADMAP.md).

## Validation

```bash
sh scripts/test-phase1-terminal.sh
```

CI validates shell syntax, launcher help/version/env/doctor output, color detection, theme previews, profile listing, self-test, benchmark smoke, install dry-run, uninstall dry-run, and required Linux/macOS assets.

## Safety

Phase1 Terminal preserves safe defaults, uses allowlisted config mutation, keeps user config unless `--remove-config` is explicit, and supports mono fallback through `NO_COLOR=1` or `PHASE1_COLOR_MODE=mono`.
