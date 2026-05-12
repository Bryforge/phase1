#!/usr/bin/env sh
# Phase1 B43 polished boot preparation wrapper.
#
# This orchestration script applies local B43 policy, builds the native binary,
# runs preflight, and delegates to the verified USB writer/verification path.
#
# IMPORTANT: do not run this whole script with sudo. It calls sudo internally
# only when media writing is required. Running the whole script with sudo hides
# the user's rustup/cargo environment and can create root-owned build logs.
#
# Usage on the x86_64 X200/final builder:
#   sh scripts/x200-b43-prepare-polished-boot.sh /dev/sdb YES_WRITE_USB

set -eu

if [ -d "$HOME/.cargo/bin" ]; then PATH="$HOME/.cargo/bin:$PATH"; export PATH; fi
if [ -f "$HOME/.cargo/env" ]; then . "$HOME/.cargo/env" 2>/dev/null || true; fi
if [ -n "${SUDO_USER:-}" ] && [ "$SUDO_USER" != root ]; then
  SUDO_HOME="$(getent passwd "$SUDO_USER" 2>/dev/null | awk -F: '{print $6}')"
  [ -n "$SUDO_HOME" ] || SUDO_HOME="/home/$SUDO_USER"
  if [ -d "$SUDO_HOME/.cargo/bin" ]; then PATH="$SUDO_HOME/.cargo/bin:$PATH"; export PATH; fi
  if [ -f "$SUDO_HOME/.cargo/env" ]; then . "$SUDO_HOME/.cargo/env" 2>/dev/null || true; fi
fi

USB="${1:-}"
CONFIRM="${2:-}"

fail() { printf 'x200-b43-prepare-polished-boot: %s\n' "$1" >&2; exit 1; }
section() { printf '\n===== %s =====\n' "$1"; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

fix_ownership_if_needed() {
  for path in \
    build/base1-b42-native-stable-safe-color-utf8 \
    build/base1-b43-system-preflight \
    target
  do
    if [ -e "$path" ] && [ ! -w "$path" ]; then
      printf 'Repairing root-owned/unwritable path: %s\n' "$path"
      sudo chown -R "$(id -u):$(id -g)" "$path"
    fi
  done
}

[ -n "$USB" ] || fail "usage: sh scripts/x200-b43-prepare-polished-boot.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from phase1 repository root"

if [ "$(id -u)" = "0" ]; then
  cat >&2 <<'EOF'
Do not run the whole B43 wrapper with sudo.

Run as your normal user:
  cd ~/phase1
  sh scripts/x200-b43-prepare-polished-boot.sh /dev/sdb YES_WRITE_USB

The script uses sudo internally only for disk writing.
EOF
  exit 1
fi

for c in git sh cargo file sudo id; do need "$c"; done

section "REPAIR LOCAL BUILD OWNERSHIP"
fix_ownership_if_needed

section "UPDATE REPOSITORY"
git fetch origin edge/stable
git pull --ff-only origin edge/stable
git log -1 --oneline

section "APPLY B43 LOCAL UI POLICY"
[ -f scripts/x200-b43-apply-ui-policy.sh ] || fail "missing scripts/x200-b43-apply-ui-policy.sh"
sh scripts/x200-b43-apply-ui-policy.sh

section "BUILD NATIVE PHASE1"
rustup default stable >/dev/null 2>&1 || true
cargo build --release
[ -x target/release/phase1 ] || fail "missing target/release/phase1 after build"
file target/release/phase1

section "ARCHITECTURE WARNING CHECK"
case "$(file target/release/phase1)" in
  *x86-64*) printf 'x86_64 Phase1 binary: ok for X200\n' ;;
  *)
    printf 'WARNING: built Phase1 binary is not x86_64.\n'
    printf 'This is acceptable for preflight on this host, but not for final X200 boot media.\n'
    ;;
esac

section "B43 PREFLIGHT"
sh scripts/x200-b43-system-preflight.sh "$USB"

section "WRITE AND VERIFY B43/B42 USB"
printf 'Delegating to verified media automation.\n'
sh scripts/x200-b40-prepare-native-phase1-boot.sh "$USB" YES_WRITE_USB

section "B43 POLISHED BOOT PREP COMPLETE"
printf 'Expected default boot entry:\n'
printf '  Start Phase1 Stable Safe Color UTF-8\n'
printf 'Expected fallback-only entry:\n'
printf '  Start Phase1 ASCII Safe Fallback\n'
printf 'Expected policy markers:\n'
printf '  PHASE1_AUTO_BOOT=1\n'
printf '  PHASE1_ASCII=0 main, PHASE1_ASCII=1 fallback\n'
printf '  safe/stable blue/ice request\n'
printf '  edge crimson request\n'
printf '  rounded glyph request\n'
