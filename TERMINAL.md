# Phase1 Terminal

Phase1 Terminal is the dedicated terminal launcher and profile layer for Phase1 on Linux and macOS.

The installed command is:

```bash
phase1-terminal
```

A shorter `terminal` alias can also be installed when safe.

## Name

The project name is **Phase1 Terminal**. The command is `phase1-terminal` because `terminal` can conflict with existing system tools, shell aliases, or user scripts. The installer can create `terminal` as an alias only when requested or when no existing command is found.

## What it does

Phase1 Terminal is not a full graphical terminal emulator. It is a Phase1-specific terminal launcher that:

- loads a Phase1 terminal configuration file
- sets Phase1-friendly environment defaults
- locates a Phase1 checkout or installed binary
- prefers an existing built binary
- falls back to `cargo run`
- sets the terminal title to `Phase1 Terminal`
- provides `doctor` and `env` diagnostics
- installs Linux and macOS launch integrations

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
phase1-terminal doctor
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
phase1-terminal doctor
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
```

## Configuration

Default config file:

```text
~/.config/phase1/terminal.env
```

Example:

```sh
PHASE1_HOME="$HOME/phase1"
PHASE1_TERMINAL_TITLE="Phase1 Terminal"
PHASE1_THEME="cyber"
PHASE1_SAFE_MODE="1"
PHASE1_MOBILE_MODE="0"
PHASE1_DEVICE_MODE="desktop"
PHASE1_ASCII="0"
```

To use a custom config file:

```bash
PHASE1_TERMINAL_CONFIG=/path/to/terminal.env phase1-terminal
```

## Commands

```bash
phase1-terminal          # launch Phase1
phase1-terminal run      # launch Phase1
phase1-terminal doctor   # check environment and Phase1 discovery
phase1-terminal env      # print effective Phase1 Terminal environment
phase1-terminal help     # show help
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

## Safety

Phase1 Terminal preserves Phase1 safe defaults:

- `PHASE1_SAFE_MODE=1`
- `PHASE1_DEVICE_MODE=desktop`
- no host-tool trust is enabled by the launcher
- no host network mutation is enabled by the launcher

Host-capable Phase1 modes must still be explicitly enabled through Phase1's normal boot/profile controls.
