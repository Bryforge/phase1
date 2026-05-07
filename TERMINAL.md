# Phase1 Terminal

Phase1 Terminal is the dedicated terminal launcher and profile layer for Phase1 on Linux and macOS.

The installed command is:

```bash
phase1-terminal
```

A shorter `terminal` alias can also be installed when safe.

Roadmap: [`TERMINAL_ROADMAP.md`](TERMINAL_ROADMAP.md).

## Name

The project name is **Phase1 Terminal**. The command is `phase1-terminal` because `terminal` can conflict with existing system tools, shell aliases, or user scripts. The installer can create `terminal` as an alias only when requested or when no existing command is found.

## What it does

Phase1 Terminal is not a full graphical terminal emulator. It is a Phase1-specific terminal management layer that:

- loads a Phase1 terminal configuration file
- sets Phase1-friendly environment defaults
- locates a Phase1 checkout or installed binary
- prefers an existing built binary
- falls back to `cargo run`
- sets the terminal title to `Phase1 Terminal`
- provides text and JSON diagnostics
- manages safe profiles and config presets
- provides build/check/log helpers
- exposes Gina/security launch shortcuts
- installs Linux and macOS launch integrations
- supports dry-run install and reversible uninstall

## Install on Linux

From the Phase1 repository root:

```bash
sh scripts/install-phase1-terminal-linux.sh
```

Then ensure `~/.local/bin` is in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Run:

```bash
phase1-terminal doctor --verbose
phase1-terminal
```

The Linux installer also adds:

```text
~/.local/share/applications/phase1-terminal.desktop
```

## Install on macOS

From the Phase1 repository root:

```bash
sh scripts/install-phase1-terminal-macos.sh
```

Then ensure `~/.local/bin` is in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Run:

```bash
phase1-terminal doctor --verbose
phase1-terminal
```

The macOS installer also creates a clickable launcher:

```text
~/Desktop/Phase1 Terminal.command
```

Optional Terminal.app profile:

```text
terminal/macos/Phase1-Terminal.terminal
```

Open that file in Terminal.app to import the profile.

## Manual install

```bash
sh scripts/install-phase1-terminal.sh
```

Options:

```bash
sh scripts/install-phase1-terminal.sh --prefix "$HOME/.local"
sh scripts/install-phase1-terminal.sh --alias
sh scripts/install-phase1-terminal.sh --no-alias
sh scripts/install-phase1-terminal.sh --phase1-home "$PWD"
sh scripts/install-phase1-terminal.sh --dry-run
```

## Uninstall

```bash
sh scripts/uninstall-phase1-terminal.sh
```

Options:

```bash
sh scripts/uninstall-phase1-terminal.sh --dry-run
sh scripts/uninstall-phase1-terminal.sh --remove-config
sh scripts/uninstall-phase1-terminal.sh --prefix "$HOME/.local"
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
PHASE1_SAFE_MODE="1"
PHASE1_MOBILE_MODE="0"
PHASE1_DEVICE_MODE="desktop"
PHASE1_ASCII="0"
PHASE1_TERMINAL_HINTS="1"
```

To use a custom config file:

```bash
PHASE1_TERMINAL_CONFIG=/path/to/terminal.env phase1-terminal
```

Config commands:

```bash
phase1-terminal config show
phase1-terminal config set PHASE1_THEME=matrix
phase1-terminal config set PHASE1_HOME="$PWD"
phase1-terminal config reset
```

Only allowlisted config keys can be changed through `config set`.

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
```

Profiles are safe by default. They change launch defaults such as theme, ASCII mode, and device mode. They do not enable host trust, host networking, or external AI providers.

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
phase1-terminal env             # print effective environment
phase1-terminal version         # print launcher version
phase1-terminal help            # show help
```

## Recommended build flow

For the fastest launch, build Phase1 once:

```bash
cargo build --release
phase1-terminal
```

If no built binary exists, Phase1 Terminal falls back to:

```bash
cargo run --
```

## Gina workflow

Gina stays offline and sandboxed by default. Phase1 Terminal makes her easier to discover:

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

## Roadmap

The terminal roadmap covers installer hardening, profile UX, session management, Gina-aware workflows, packaging, and future native terminal exploration.

Read: [`TERMINAL_ROADMAP.md`](TERMINAL_ROADMAP.md).

## Validation

```bash
sh scripts/test-phase1-terminal.sh
```

CI validates shell syntax, launcher help/version/env/doctor output, profile listing, install dry-run, uninstall dry-run, and required Linux/macOS assets.

## Safety

Phase1 Terminal preserves Phase1 safe defaults:

- `PHASE1_SAFE_MODE=1`
- `PHASE1_DEVICE_MODE=desktop`
- no host-tool trust is enabled by the launcher
- no host network mutation is enabled by the launcher
- no external AI provider is enabled by the launcher
- config changes are allowlisted
- uninstall keeps user config unless `--remove-config` is explicit

Host-capable Phase1 modes must still be explicitly enabled through Phase1's normal boot/profile controls.
