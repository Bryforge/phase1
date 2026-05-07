#!/usr/bin/env bash
set -euo pipefail

# Phase1 Raspberry Pi 5 compatibility launcher.
# This mode favors terminal correctness over visuals:
# - cooked input instead of raw stty editing
# - ASCII-safe boot/prompt rendering
# - stable terminal width
# - no truecolor assumptions
# - no host tools unless the operator enables them separately

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

export LC_ALL="${LC_ALL:-C.UTF-8}"
export LANG="${LANG:-C.UTF-8}"
export TERM="${TERM:-xterm-256color}"
export COLUMNS="${COLUMNS:-80}"

export PHASE1_PLATFORM="raspberry-pi-5"
export PHASE1_DEVICE_MODE="raspberry-pi"
export PHASE1_RPI_COMPAT="1"
export PHASE1_TERMINAL_COMPAT="1"
export PHASE1_COOKED_INPUT="1"
export PHASE1_ASCII="1"
export PHASE1_FORCE_ASCII="1"
export PHASE1_NO_COLOR="1"
export PHASE1_COLOR_PACK="raspberry-pi"
export PHASE1_COLOR_DEPTH="mono"
export PHASE1_THEME="mono"
export PHASE1_QUICK_BOOT="1"
export PHASE1_SAFE_MODE="1"
export PHASE1_IDLE_ENTER_GUARD_SECONDS="0"

printf 'phase1 rpi5 compatibility mode\n'
printf 'input    : cooked line mode\n'
printf 'display  : ascii/mono safe output\n'
printf 'terminal : TERM=%s COLUMNS=%s\n' "$TERM" "$COLUMNS"
printf 'security : safe mode on; host tools remain off unless explicitly enabled\n\n'

exec cargo run -- "$@"
