#!/usr/bin/env sh
set -eu

PREFIX="${PREFIX:-$HOME/.local}"
CREATE_ALIAS="${CREATE_ALIAS:-auto}"
PHASE1_HOME_VALUE="${PHASE1_HOME:-$(pwd)}"
BIN_DIR="$PREFIX/bin"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/phase1"
CONFIG_FILE="$CONFIG_DIR/terminal.env"
LAUNCHER_SOURCE="terminal/bin/phase1-terminal"
LAUNCHER_TARGET="$BIN_DIR/phase1-terminal"
ALIAS_TARGET="$BIN_DIR/terminal"

usage() {
    cat <<'EOF'
Install Phase1 Terminal

Usage:
  sh scripts/install-phase1-terminal.sh [options]

Options:
  --prefix PATH       Install prefix. Default: $HOME/.local
  --alias             Also create a terminal alias/symlink.
  --no-alias          Do not create a terminal alias/symlink.
  --phase1-home PATH  Set PHASE1_HOME in generated config.
  -h, --help          Show help.

Environment:
  PREFIX              Install prefix override.
  CREATE_ALIAS        auto, yes, or no. Default: auto.
  PHASE1_HOME         Phase1 repository/install path.
EOF
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --prefix)
            PREFIX="$2"
            BIN_DIR="$PREFIX/bin"
            LAUNCHER_TARGET="$BIN_DIR/phase1-terminal"
            ALIAS_TARGET="$BIN_DIR/terminal"
            shift 2
            ;;
        --alias)
            CREATE_ALIAS="yes"
            shift
            ;;
        --no-alias)
            CREATE_ALIAS="no"
            shift
            ;;
        --phase1-home)
            PHASE1_HOME_VALUE="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "unknown option: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

if [ ! -f "$LAUNCHER_SOURCE" ]; then
    echo "installer must be run from the Phase1 repository root" >&2
    exit 1
fi

mkdir -p "$BIN_DIR" "$CONFIG_DIR"
cp "$LAUNCHER_SOURCE" "$LAUNCHER_TARGET"
chmod 0755 "$LAUNCHER_TARGET"

cat > "$CONFIG_FILE" <<EOF
# Phase1 Terminal config
PHASE1_HOME="$PHASE1_HOME_VALUE"
PHASE1_TERMINAL_TITLE="Phase1 Terminal"
PHASE1_THEME="cyber"
PHASE1_SAFE_MODE="1"
PHASE1_MOBILE_MODE="0"
PHASE1_DEVICE_MODE="desktop"
PHASE1_ASCII="0"
EOF

case "$CREATE_ALIAS" in
    yes)
        ln -sf "$LAUNCHER_TARGET" "$ALIAS_TARGET"
        alias_status="created"
        ;;
    no)
        alias_status="skipped"
        ;;
    auto)
        if command -v terminal >/dev/null 2>&1; then
            alias_status="skipped; terminal command already exists"
        else
            ln -sf "$LAUNCHER_TARGET" "$ALIAS_TARGET"
            alias_status="created"
        fi
        ;;
    *)
        echo "CREATE_ALIAS must be auto, yes, or no" >&2
        exit 1
        ;;
esac

case ":$PATH:" in
    *":$BIN_DIR:"*) path_hint="already in PATH" ;;
    *) path_hint="add $BIN_DIR to PATH" ;;
esac

cat <<EOF
Phase1 Terminal installed.

Command : $LAUNCHER_TARGET
Alias   : $alias_status
Config  : $CONFIG_FILE
PATH    : $path_hint

Try:
  phase1-terminal doctor
  phase1-terminal
EOF
