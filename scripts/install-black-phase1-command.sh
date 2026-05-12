#!/usr/bin/env sh
# Install a local black-phase1 command that routes to scripts/black-phase1.sh.
#
# Usage:
#   sh scripts/install-black-phase1-command.sh
#
# Installs:
#   ~/.local/bin/black-phase1
#
# Then use:
#   black-phase1 status
#   black-phase1 doctor /dev/sdb
#   black-phase1 x200-test /dev/sdb YES_WRITE_USB

set -eu

fail() { printf 'install-black-phase1-command: %s\n' "$1" >&2; exit 1; }
[ -d .git ] || fail "run from phase1 repository root"
[ -f scripts/black-phase1.sh ] || fail "missing scripts/black-phase1.sh"

INSTALL_DIR="${BLACK_PHASE1_BIN_DIR:-$HOME/.local/bin}"
CMD="$INSTALL_DIR/black-phase1"
REPO="$(pwd)"

mkdir -p "$INSTALL_DIR"

cat > "$CMD" <<EOF
#!/usr/bin/env sh
cd "$REPO" || exit 1
exec sh scripts/black-phase1.sh "\$@"
EOF
chmod +x "$CMD"

printf 'Installed black-phase1 command:\n'
printf '  %s\n\n' "$CMD"

case ":$PATH:" in
  *":$INSTALL_DIR:"*)
    printf 'PATH already includes %s\n' "$INSTALL_DIR"
    ;;
  *)
    printf 'Add this to your shell config if needed:\n'
    printf '  export PATH="%s:$PATH"\n' "$INSTALL_DIR"
    ;;
esac

printf '\nTry:\n'
printf '  black-phase1 status\n'
