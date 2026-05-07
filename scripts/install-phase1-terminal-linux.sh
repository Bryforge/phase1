#!/usr/bin/env sh
set -eu

PREFIX="${PREFIX:-$HOME/.local}"
DESKTOP_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/applications"

sh scripts/install-phase1-terminal.sh --prefix "$PREFIX" "$@"

mkdir -p "$DESKTOP_DIR"
cp terminal/linux/phase1-terminal.desktop "$DESKTOP_DIR/phase1-terminal.desktop"
chmod 0644 "$DESKTOP_DIR/phase1-terminal.desktop"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$DESKTOP_DIR" >/dev/null 2>&1 || true
fi

cat <<EOF
Linux desktop launcher installed:
  $DESKTOP_DIR/phase1-terminal.desktop
EOF
