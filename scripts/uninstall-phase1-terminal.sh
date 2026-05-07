#!/usr/bin/env sh
set -eu

PREFIX="${PREFIX:-$HOME/.local}"
DRY_RUN="0"
REMOVE_CONFIG="0"
BIN_DIR="$PREFIX/bin"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/phase1"
CONFIG_FILE="$CONFIG_DIR/terminal.env"
DESKTOP_FILE="${XDG_DATA_HOME:-$HOME/.local/share}/applications/phase1-terminal.desktop"
MACOS_SUPPORT="$HOME/Library/Application Support/Phase1/Phase1 Terminal.command"
MACOS_DESKTOP="$HOME/Desktop/Phase1 Terminal.command"
LAUNCHER_TARGET="$BIN_DIR/phase1-terminal"
ALIAS_TARGET="$BIN_DIR/terminal"

usage() {
    cat <<'EOF'
Uninstall Phase1 Terminal

Usage:
  sh scripts/uninstall-phase1-terminal.sh [options]

Options:
  --prefix PATH       Install prefix. Default: $HOME/.local
  --remove-config     Also remove ~/.config/phase1/terminal.env.
  --dry-run           Print planned removals without deleting files.
  -h, --help          Show help.
EOF
}

refresh_paths() {
    BIN_DIR="$PREFIX/bin"
    LAUNCHER_TARGET="$BIN_DIR/phase1-terminal"
    ALIAS_TARGET="$BIN_DIR/terminal"
}

remove_path() {
    path="$1"
    if [ -e "$path" ] || [ -L "$path" ]; then
        if [ "$DRY_RUN" = "1" ]; then
            echo "dry-run: remove $path"
        else
            rm -f "$path"
            echo "removed: $path"
        fi
    fi
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --prefix)
            PREFIX="$2"
            refresh_paths
            shift 2
            ;;
        --remove-config)
            REMOVE_CONFIG="1"
            shift
            ;;
        --dry-run)
            DRY_RUN="1"
            shift
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

remove_path "$LAUNCHER_TARGET"

if [ -L "$ALIAS_TARGET" ]; then
    target=$(readlink "$ALIAS_TARGET" || true)
    if [ "$target" = "$LAUNCHER_TARGET" ]; then
        remove_path "$ALIAS_TARGET"
    else
        echo "skipped alias: $ALIAS_TARGET points to $target"
    fi
fi

remove_path "$DESKTOP_FILE"
remove_path "$MACOS_SUPPORT"
remove_path "$MACOS_DESKTOP"

if [ "$REMOVE_CONFIG" = "1" ]; then
    remove_path "$CONFIG_FILE"
else
    echo "config kept: $CONFIG_FILE"
fi

if [ "$DRY_RUN" = "1" ]; then
    echo "Phase1 Terminal uninstall dry-run complete."
else
    echo "Phase1 Terminal uninstall complete."
fi
