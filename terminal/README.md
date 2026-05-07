# Phase1 Terminal

Phase1 Terminal is the dedicated launcher/profile layer for Phase1 on Linux and macOS.

Use it when you want a consistent Phase1 entry point that configures the shell environment, discovers the Phase1 install path, and launches the fastest available Phase1 binary.

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
├── bin/phase1-terminal              # launcher command
├── linux/phase1-terminal.desktop    # Linux desktop entry template
└── macos/Phase1-Terminal.terminal   # optional Terminal.app profile
```

Installer scripts live in `scripts/`:

```text
scripts/install-phase1-terminal.sh
scripts/install-phase1-terminal-linux.sh
scripts/install-phase1-terminal-macos.sh
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
PHASE1_THEME="cyber"
PHASE1_SAFE_MODE="1"
PHASE1_DEVICE_MODE="desktop"
```

## Roadmap

The Phase1 Terminal roadmap covers installer hardening, profile UX, session management, Gina-aware workflows, packaging, and native terminal exploration.

Read: [`../TERMINAL_ROADMAP.md`](../TERMINAL_ROADMAP.md).

## Safety model

Phase1 Terminal does not enable host-trust or host-network mutation settings. It launches Phase1 with safe defaults and leaves higher-trust modes to Phase1's normal boot controls.

See `TERMINAL.md` for full usage details.
