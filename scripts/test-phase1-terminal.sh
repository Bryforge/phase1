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

echo "phase1-terminal doctor --json"
sh terminal/bin/phase1-terminal doctor --json | grep '"version"'

echo "phase1-terminal profile list"
sh terminal/bin/phase1-terminal profile list | grep 'profiles:'

echo "phase1-terminal install dry-run"
sh scripts/install-phase1-terminal.sh --dry-run --no-alias >/dev/null

echo "phase1-terminal uninstall dry-run"
sh scripts/uninstall-phase1-terminal.sh --dry-run >/dev/null

echo "Phase1 Terminal scripts validated"
