#!/usr/bin/env sh
set -eu

for file in \
    terminal/bin/phase1-terminal \
    scripts/install-phase1-terminal.sh \
    scripts/install-phase1-terminal-linux.sh \
    scripts/install-phase1-terminal-macos.sh
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

echo "Phase1 Terminal scripts validated"
