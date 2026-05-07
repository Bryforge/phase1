#!/usr/bin/env sh
set -eu

PREFIX="${PREFIX:-$HOME/.local}"
DESKTOP_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/applications"

sh scripts/install-phase1-terminal.sh --prefix "$PREFIX" "$@"

mkdir -p "$DESKTOP_DIR"
cat > "$DESKTOP_DIR/phase1-terminal.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Phase1 Terminal
Comment=Launch the Phase1 terminal-first computing environment
Exec=$PREFIX/bin/phase1-terminal
Terminal=true
Categories=Development;System;TerminalEmulator;
Keywords=phase1;terminal;rust;os;shell;
StartupNotify=false
EOF
chmod 0644 "$DESKTOP_DIR/phase1-terminal.desktop"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$DESKTOP_DIR" >/dev/null 2>&1 || true
fi

cat <<EOF
Linux desktop launcher installed:
  $DESKTOP_DIR/phase1-terminal.desktop
EOF
