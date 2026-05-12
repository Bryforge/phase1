#!/usr/bin/env sh
# Phase1 X200 boot automation, backward-compatible filename.
#
# Current target: B42 stable-safe native color UTF-8 boot.
# This script now uses the B42 writer, verifies the written USB, and fails if
# the old B40/B41 menu is still present.
#
# Usage:
#   sh scripts/x200-b40-prepare-native-phase1-boot.sh /dev/sdb YES_WRITE_USB

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
WRITER="scripts/x200-b42-native-stable-safe-color-utf8-usb.sh"
KERNEL="${BASE1_B42_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
SPLASH="${BASE1_B42_SPLASH:-assets/phase1-splash.png}"
PHASE1_BIN="${BASE1_B42_PHASE1_BIN:-target/release/phase1}"
BUSYBOX="${BASE1_B42_BUSYBOX:-}"
REPORT_DIR="build/base1-b42-native-stable-safe-color-utf8"
REPORT="$REPORT_DIR/b42-automation-diagnostics.env"
MAIN_ENTRY="Start Phase1 Stable Safe Color UTF-8"
ASCII_ENTRY="Start Phase1 ASCII Safe Fallback"
INITRD_NAME="phase1-b42-native-stable-safe-color-utf8.img"

fail() { printf 'phase1-b42-automation: %s\n' "$1" >&2; exit 1; }
section() { printf '\n%s\n' "$1"; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

part1() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole disk like /dev/sdb" ;;
  esac
}

find_busybox() {
  if [ -n "$BUSYBOX" ]; then printf '%s\n' "$BUSYBOX"; return; fi
  for p in /bin/busybox /usr/bin/busybox /bin/busybox.static /usr/bin/busybox.static; do
    [ -x "$p" ] && { printf '%s\n' "$p"; return; }
  done
  printf '\n'
}

[ -n "$USB" ] || fail "usage: sh scripts/x200-b40-prepare-native-phase1-boot.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = YES_WRITE_USB ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from the phase1 repository root"
for c in sh git sudo grep awk sha256sum cargo mount umount mktemp; do need "$c"; done
mkdir -p "$REPORT_DIR"

section "--- updating repository ---"
git fetch origin edge/stable
git pull --ff-only origin edge/stable

section "--- checking current B42 writer ---"
[ -f "$WRITER" ] || fail "missing $WRITER"
grep -q "$MAIN_ENTRY" "$WRITER" || fail "writer missing main B42 entry: $MAIN_ENTRY"
grep -q "$ASCII_ENTRY" "$WRITER" || fail "writer missing ASCII fallback entry: $ASCII_ENTRY"
grep -q 'BASE1_B42_ASCII_DEFAULT=0' "$WRITER" || fail "writer missing ASCII default=0 evidence"
grep -q 'BASE1_B42_SAFE_DEFAULT=1' "$WRITER" || fail "writer missing safe default=1 evidence"

git log -1 --oneline -- "$WRITER" || true

section "--- building native Phase1 binary ---"
rustup default stable >/dev/null 2>&1 || true
cargo build --release
[ -x "$PHASE1_BIN" ] || fail "missing built binary: $PHASE1_BIN"
sha256sum "$PHASE1_BIN"

section "--- checking kernel, splash, busybox ---"
[ -f "$KERNEL" ] || sh scripts/x200-b23-stage-host-gnulinux.sh
[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL"
[ -f "$SPLASH" ] || fail "missing splash: $SPLASH"
BUSYBOX_PATH="$(find_busybox)"
[ -n "$BUSYBOX_PATH" ] || fail "missing busybox-static"
sha256sum "$KERNEL" "$SPLASH" "$BUSYBOX_PATH" 2>/dev/null || true

section "--- writing B42 USB ---"
sudo env \
  BASE1_B42_PHASE1_BIN="$PWD/$PHASE1_BIN" \
  BASE1_B42_KERNEL="$PWD/$KERNEL" \
  BASE1_B42_SPLASH="$PWD/$SPLASH" \
  BASE1_B42_BUSYBOX="$BUSYBOX_PATH" \
  sh "$WRITER" "$USB" YES_WRITE_USB

section "--- verifying USB after write ---"
PART="$(part1 "$USB")"
MNT="$(mktemp -d)"
sudo mount -o ro "$PART" "$MNT"
trap 'sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true' EXIT INT TERM
[ -f "$MNT/boot/grub/grub.cfg" ] || fail "USB missing grub.cfg"
[ -f "$MNT/boot/phase1/vmlinuz" ] || fail "USB missing vmlinuz"
[ -f "$MNT/boot/phase1/$INITRD_NAME" ] || fail "USB missing $INITRD_NAME"
[ -f "$MNT/phase1/assets/phase1-splash.png" ] || fail "USB missing real splash"
[ -f "$MNT/phase1/evidence/b42-prep.env" ] || fail "USB missing b42-prep.env"
grep -q "$MAIN_ENTRY" "$MNT/boot/grub/grub.cfg" || fail "USB missing main B42 entry"
grep -q "$ASCII_ENTRY" "$MNT/boot/grub/grub.cfg" || fail "USB missing ASCII fallback entry"
grep -q 'phase1.ascii=0' "$MNT/boot/grub/grub.cfg" || fail "USB default entry does not pass phase1.ascii=0"
grep -q 'phase1.ascii=1' "$MNT/boot/grub/grub.cfg" || fail "USB fallback entry does not pass phase1.ascii=1"
grep -q 'BASE1_B42_ASCII_DEFAULT=0' "$MNT/phase1/evidence/b42-prep.env" || fail "USB evidence does not show ascii default off"

echo "USB menu entries:"
grep '^menuentry ' "$MNT/boot/grub/grub.cfg" || true

echo "USB evidence:"
cat "$MNT/phase1/evidence/b42-prep.env"
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B42_AUTOMATION_TARGET=$USB
BASE1_B42_AUTOMATION_PARTITION=$PART
BASE1_B42_AUTOMATION_WRITER=$WRITER
BASE1_B42_AUTOMATION_MAIN_ENTRY=$MAIN_ENTRY
BASE1_B42_AUTOMATION_ASCII_ENTRY=$ASCII_ENTRY
BASE1_B42_AUTOMATION_ASCII_DEFAULT=0
BASE1_B42_AUTOMATION_ASCII_FALLBACK=1
BASE1_B42_AUTOMATION_RESULT=prepared_and_verified
BASE1_B42_AUTOMATION_EXPECTED_RESULT=phase1_native_color_console_seen
BASE1_B42_AUTOMATION_UTF8_RESULT=phase1_japanese_utf8_ready
BASE1_B42_AUTOMATION_STABLE_RESULT=phase1_stable_safe_defaults_seen
EOF

printf '\nDONE: B42 USB prepared and verified.\n'
printf 'Default boot: %s\n' "$MAIN_ENTRY"
printf 'Fallback only: %s\n' "$ASCII_ENTRY"
printf 'Report: %s\n' "$REPORT"
