#!/usr/bin/env sh
set -eu

PREFIX="${PREFIX:-$HOME/.local}"
APP_SUPPORT="$HOME/Library/Application Support/Phase1"
COMMAND_FILE="$HOME/Desktop/Phase1 Terminal.command"

sh scripts/install-phase1-terminal.sh --prefix "$PREFIX" --no-alias "$@"

mkdir -p "$APP_SUPPORT"
cat > "$APP_SUPPORT/Phase1 Terminal.command" <<'EOF'
#!/usr/bin/env sh
if command -v phase1-terminal >/dev/null 2>&1; then
    exec phase1-terminal
fi
if [ -x "$HOME/.local/bin/phase1-terminal" ]; then
    exec "$HOME/.local/bin/phase1-terminal"
fi
echo "phase1-terminal was not found. Add ~/.local/bin to PATH or reinstall Phase1 Terminal."
read -r _
EOF
chmod 0755 "$APP_SUPPORT/Phase1 Terminal.command"
cp "$APP_SUPPORT/Phase1 Terminal.command" "$COMMAND_FILE"
chmod 0755 "$COMMAND_FILE"

cat <<EOF
macOS launcher installed.

Clickable launcher:
  $COMMAND_FILE

Support copy:
  $APP_SUPPORT/Phase1 Terminal.command

Optional: import terminal/macos/Phase1-Terminal.terminal into Terminal.app profiles.
EOF
