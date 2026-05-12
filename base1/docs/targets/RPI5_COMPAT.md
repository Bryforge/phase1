# Raspberry Pi 5 compatibility mode

Phase1 includes a Raspberry Pi 5 compatibility launcher for terminals that have trouble with raw keyboard mode, animated redraws, truecolor output, cursor motion, or narrow terminal behavior.

This mode favors reliability over visual effects so Phase1 remains usable in SSH sessions, serial consoles, framebuffer consoles, and lightweight terminal emulators.

## Start Phase1 on a Raspberry Pi 5

```bash
bash scripts/phase1-rpi5.sh
```

The launcher sets compatibility-safe defaults:

```text
PHASE1_RPI_COMPAT=1
PHASE1_TERMINAL_COMPAT=1
PHASE1_COOKED_INPUT=1
PHASE1_ASCII=1
PHASE1_FORCE_ASCII=1
PHASE1_NO_COLOR=1
PHASE1_COLOR_PACK=raspberry-pi
PHASE1_COLOR_DEPTH=mono
PHASE1_THEME=mono
PHASE1_QUICK_BOOT=1
PHASE1_SAFE_MODE=1
PHASE1_IDLE_ENTER_GUARD_SECONDS=0
COLUMNS=80
TERM=xterm-256color
```

## What it fixes

- Uses cooked line input instead of raw `stty` input.
- Reduces cursor repositioning and redraw issues.
- Avoids truecolor assumptions.
- Uses ASCII-safe prompt and boot output.
- Disables terminal effects that can glitch on low-power or basic terminals.
- Keeps safe mode on by default.

## Try color after stability

Inside Phase1:

```text
theme linux raspberry-pi
theme linux preview
```

If the terminal becomes messy again, exit and restart with:

```bash
bash scripts/phase1-rpi5.sh
```

For basic serial consoles:

```bash
TERM=vt100 COLUMNS=80 bash scripts/phase1-rpi5.sh
```
