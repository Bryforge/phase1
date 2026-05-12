#!/usr/bin/env sh
# Phase1 B43 polished boot preparation wrapper.
#
# Branch-aware version: updates the current branch or BASE1_TARGET_BRANCH
# instead of forcing edge/stable. This keeps black-phase1 experiments from
# diverging/failing during rapid cycles.

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
TARGET_BRANCH="${BASE1_TARGET_BRANCH:-$(git rev-parse --abbrev-ref HEAD 2>/dev/null || printf black-phase1)}"

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
printf 'target branch: %s\n' "$TARGET_BRANCH"
git fetch origin "$TARGET_BRANCH"
git pull --ff-only origin "$TARGET_BRANCH"
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
BASE1_TARGET_BRANCH="$TARGET_BRANCH" sh scripts/x200-b40-prepare-native-phase1-boot.sh "$USB" YES_WRITE_USB

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
