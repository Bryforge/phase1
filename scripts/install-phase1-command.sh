#!/usr/bin/env sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
INSTALL_DIR="${PHASE1_BIN_DIR:-$HOME/.local/bin}"
COMMAND_PATH="$INSTALL_DIR/phase1"
PHASE1_ENTRY="$ROOT_DIR/phase1"

usage() {
    cat <<'EOF'
Install Phase1 command

Usage:
  sh scripts/install-phase1-command.sh

Environment:
  PHASE1_BIN_DIR=/custom/bin  Install command somewhere else.

After install:
  phase1
  phase1 doctor
  phase1 version

Notes:
  The installed command is a tiny wrapper back to this checkout.
  It does not move the repo or create a new trust path.
EOF
}

case "${1:-}" in
    -h|--help)
        usage
        exit 0
        ;;
    "") ;;
    *)
        echo "unknown option: $1" >&2
        usage >&2
        exit 1
        ;;
esac

if [ ! -f "$PHASE1_ENTRY" ]; then
    echo "install-phase1-command: missing root launcher: $PHASE1_ENTRY" >&2
    exit 1
fi

mkdir -p "$INSTALL_DIR"
cat > "$COMMAND_PATH" <<EOF
#!/usr/bin/env sh
exec "$PHASE1_ENTRY" "\$@"
EOF
chmod 0755 "$COMMAND_PATH"

printf 'Installed Phase1 command: %s\n' "$COMMAND_PATH"
printf 'Try: %s doctor\n' "$COMMAND_PATH"

case ":${PATH}:" in
    *":$INSTALL_DIR:"*)
        printf 'PATH ready: %s is already available.\n' "$INSTALL_DIR"
        ;;
    *)
        printf '\nAdd this to your shell profile if `phase1` is not found:\n'
        printf '  export PATH="$HOME/.local/bin:$PATH"\n'
        printf '\nThen open a new terminal or run:\n'
        printf '  export PATH="$HOME/.local/bin:$PATH"\n'
        ;;
esac
