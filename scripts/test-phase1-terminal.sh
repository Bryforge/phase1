#!/usr/bin/env sh
set -eu

for file in \
    terminal/bin/phase1-terminal \
    scripts/install-phase1-terminal.sh \
    scripts/install-phase1-terminal-linux.sh \
    scripts/install-phase1-terminal-macos.sh \
    scripts/uninstall-phase1-terminal.sh
 do
    echo "sh -n $file"
    sh -n "$file"
 done

if [ ! -f terminal/linux/phase1-terminal.desktop ]; then
    echo "missing Linux desktop entry" >&2
    exit 1
fi

if [ ! -f terminal/macos/Phase1-Terminal.terminal ]; then
    echo "missing macOS Terminal profile" >&2
    exit 1
fi

echo "phase1-terminal help"
sh terminal/bin/phase1-terminal help >/dev/null

echo "phase1-terminal version"
sh terminal/bin/phase1-terminal version | grep 'phase1-terminal '

echo "phase1-terminal env"
sh terminal/bin/phase1-terminal env | grep 'PHASE1_TERMINAL_VERSION='
sh terminal/bin/phase1-terminal env | grep 'PHASE1_DETECTED_COLOR_MODE='

echo "phase1-terminal doctor --json"
sh terminal/bin/phase1-terminal doctor --json | grep '"version"'
sh terminal/bin/phase1-terminal doctor --json | grep '"detected_color_mode"'

echo "phase1-terminal colors detect"
sh terminal/bin/phase1-terminal colors detect | grep 'detected'

echo "phase1-terminal colors swatches"
sh terminal/bin/phase1-terminal colors swatches | grep 'cyber'

echo "phase1-terminal theme list"
sh terminal/bin/phase1-terminal theme list | grep 'themes:'

echo "phase1-terminal theme preview matrix"
sh terminal/bin/phase1-terminal theme preview matrix | grep 'matrix'

echo "phase1-terminal profile list"
sh terminal/bin/phase1-terminal profile list | grep 'profiles:'

echo "phase1-terminal selftest"
sh terminal/bin/phase1-terminal selftest | grep 'selftest: passed'

echo "phase1-terminal benchmark"
sh terminal/bin/phase1-terminal benchmark 2 | grep 'status    : completed'

echo "phase1-terminal install dry-run"
sh scripts/install-phase1-terminal.sh --dry-run --no-alias >/dev/null

echo "phase1-terminal uninstall dry-run"
sh scripts/uninstall-phase1-terminal.sh --dry-run >/dev/null

echo "Phase1 Terminal scripts validated"
