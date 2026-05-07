# Phase1 Terminal

Phase1 Terminal is the dedicated launcher/profile/config/session layer for Phase1 on Linux and macOS.

Use it when you want a consistent Phase1 entry point that configures the shell environment, discovers the Phase1 install path, launches the fastest available Phase1 binary, and provides safe management commands around the Phase1 terminal experience.

Roadmap: [`../TERMINAL_ROADMAP.md`](../TERMINAL_ROADMAP.md).

## Command

```bash
phase1-terminal
```

Optional alias:

```bash
terminal
```

The project name is **Phase1 Terminal**. The installed command is `phase1-terminal` to avoid clashing with existing system commands. The installer can create a `terminal` alias when safe.

## Layout

```text
terminal/
├── bin/phase1-terminal              # launcher/config/session command
├── linux/phase1-terminal.desktop    # Linux desktop entry template
└── macos/Phase1-Terminal.terminal   # optional Terminal.app profile
```

Installer scripts live in `scripts/`:

```text
scripts/install-phase1-terminal.sh
scripts/install-phase1-terminal-linux.sh
scripts/install-phase1-terminal-macos.sh
scripts/uninstall-phase1-terminal.sh
scripts/test-phase1-terminal.sh
```

## Install

Linux:

```bash
sh scripts/install-phase1-terminal-linux.sh
```

macOS:

```bash
sh scripts/install-phase1-terminal-macos.sh
```

Generic:

```bash
sh scripts/install-phase1-terminal.sh
```

Dry-run:

```bash
sh scripts/install-phase1-terminal.sh --dry-run
```

## Uninstall

```bash
sh scripts/uninstall-phase1-terminal.sh
```

Dry-run:

```bash
sh scripts/uninstall-phase1-terminal.sh --dry-run
```

Remove config explicitly:

```bash
sh scripts/uninstall-phase1-terminal.sh --remove-config
```

## Validate

```bash
sh scripts/test-phase1-terminal.sh
```

## Configure

Default config file:

```text
~/.config/phase1/terminal.env
```

Useful values:

```sh
PHASE1_HOME="$HOME/phase1"
PHASE1_TERMINAL_TITLE="Phase1 Terminal"
PHASE1_TERMINAL_PROFILE="default"
PHASE1_THEME="cyber"
PHASE1_SAFE_MODE="1"
PHASE1_DEVICE_MODE="desktop"
PHASE1_TERMINAL_HINTS="1"
```

Config commands:

```bash
phase1-terminal config show
phase1-terminal config set PHASE1_THEME=matrix
phase1-terminal config reset
```

## Profiles

```bash
phase1-terminal profile list
phase1-terminal profile apply cyber
phase1-terminal profile apply matrix
phase1-terminal profile apply mono
phase1-terminal profile apply safe
phase1-terminal profile apply developer
phase1-terminal profile apply base1
```

## Session commands

```bash
phase1-terminal run
phase1-terminal safe
phase1-terminal dev
phase1-terminal demo
phase1-terminal base1
phase1-terminal gina
phase1-terminal security
```

## Management commands

```bash
phase1-terminal doctor --verbose
phase1-terminal doctor --json
phase1-terminal build
phase1-terminal check
phase1-terminal logs
phase1-terminal env
phase1-terminal version
```

## Roadmap

The Phase1 Terminal roadmap covers installer hardening, profile UX, session management, Gina-aware workflows, packaging, and native terminal exploration.

Read: [`../TERMINAL_ROADMAP.md`](../TERMINAL_ROADMAP.md).

## Safety model

Phase1 Terminal does not enable host-trust, host-network mutation, or external AI provider settings. It launches Phase1 with safe defaults and leaves higher-trust modes to Phase1's normal boot controls.

See `TERMINAL.md` for full usage details.
