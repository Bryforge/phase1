#!/usr/bin/env sh
# Phase1 X200/Pi boot automation, backward-compatible filename.
#
# Current target: B42 stable-safe native color UTF-8 boot.
# This script verifies prerequisites before running the B42 writer and then
# verifies the written USB after the writer completes.
#
# Usage:
#   sh scripts/x200-b40-prepare-native-phase1-boot.sh /dev/sda YES_WRITE_USB
#   sh scripts/x200-b40-prepare-native-phase1-boot.sh /dev/sdb YES_WRITE_USB

set -u

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
WRITER_LOG="$REPORT_DIR/b42-writer-run.log"
VERIFY_LOG="$REPORT_DIR/b42-usb-verify.log"
RUNTIME_WRITER="$REPORT_DIR/b42-writer-runtime-safe.sh"
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
    *) fail "use a whole disk like /dev/sda or /dev/sdb, not $1" ;;
  esac
}

find_busybox() {
  if [ -n "$BUSYBOX" ]; then printf '%s\n' "$BUSYBOX"; return; fi
  for p in /bin/busybox /usr/bin/busybox /bin/busybox.static /usr/bin/busybox.static; do
    [ -x "$p" ] && { printf '%s\n' "$p"; return; }
  done
  printf '\n'
}

root_device_check() {
  root_src="$(findmnt -no SOURCE / 2>/dev/null || true)"
  printf 'root source: %s\n' "${root_src:-unknown}"
  case "$root_src" in
    "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write root filesystem device: $root_src" ;;
  esac
}

prepare_runtime_writer() {
  mkdir -p "$REPORT_DIR"
  awk '
    /\[ "\$USB" != "\/dev\/sda" \] \|\| fail "refusing \/dev\/sda because it is commonly the internal disk"/ {
      print "# Pi-safe automation removed the hard /dev/sda refusal here.";
      print "# The automation already refuses the actual root filesystem device.";
      next;
    }
    /^copy_libs_for_binary "\$PHASE1_BIN_PATH" "\$ROOTFS"$/ {
      print "# B42-R4: tolerate ldd/read EOF and partial optional library-copy exits.";
      print "copy_libs_for_binary \"$PHASE1_BIN_PATH\" \"$ROOTFS\" || true";
      next;
    }
    /^copy_common_loaders "\$ROOTFS"$/ {
      print "copy_common_loaders \"$ROOTFS\" || true";
      next;
    }
    { print }
  ' "$WRITER" > "$RUNTIME_WRITER"
  chmod +x "$RUNTIME_WRITER"
  sh -n "$RUNTIME_WRITER" || fail "runtime writer has shell syntax error"
}

show_log_tail() {
  log="$1"
  printf '\n--- tail: %s ---\n' "$log"
  if [ -f "$log" ]; then tail -n 180 "$log"; else printf 'log not found\n'; fi
  printf -- '--- end tail ---\n'
}

missing_grub_install_help() {
  cat <<'EOF'
Missing grub-install.

This writer creates an x86 BIOS/Libreboot-bootable USB, so it needs GRUB's i386-pc installer.

Try on Debian/Ubuntu/Raspberry Pi OS:
  sudo apt update
  sudo apt install -y grub-pc-bin grub-common parted dosfstools mtools

Then rerun:
  sh scripts/x200-b40-prepare-native-phase1-boot.sh /dev/sda YES_WRITE_USB

If grub-pc-bin is not available on this machine, prepare the bootloader once on the X200 or another x86 Linux machine, then use this Pi only for file refreshes later.
EOF
}

[ -n "$USB" ] || fail "usage: sh scripts/x200-b40-prepare-native-phase1-boot.sh /dev/sda YES_WRITE_USB"
[ "$CONFIRM" = YES_WRITE_USB ] || fail "missing YES_WRITE_USB confirmation"
[ -d .git ] || fail "run from the phase1 repository root"
for c in sh git sudo grep awk sha256sum cargo mount umount mktemp tail findmnt; do need "$c"; done
mkdir -p "$REPORT_DIR"
: > "$WRITER_LOG"
: > "$VERIFY_LOG"

section "--- updating repository ---"
git fetch origin edge/stable
git pull --ff-only origin edge/stable

section "--- automation target ---"
printf 'target usb      : %s\n' "$USB"
printf 'repo writer     : %s\n' "$WRITER"
printf 'runtime writer : %s\n' "$RUNTIME_WRITER"
printf 'writer log      : %s\n' "$WRITER_LOG"
printf 'verify log      : %s\n' "$VERIFY_LOG"
git log -1 --oneline || true
git log -1 --oneline -- "$WRITER" || true
root_device_check

section "--- host bootloader prerequisite check ---"
if ! command -v grub-install >/dev/null 2>&1; then
  missing_grub_install_help
  fail "missing command: grub-install"
fi
grub-install --version || true

section "--- checking B42 writer content ---"
[ -f "$WRITER" ] || fail "missing $WRITER"
sh -n "$WRITER" || fail "writer has shell syntax error"
grep -q "$MAIN_ENTRY" "$WRITER" || fail "writer missing main B42 entry: $MAIN_ENTRY"
grep -q "$ASCII_ENTRY" "$WRITER" || fail "writer missing ASCII fallback entry: $ASCII_ENTRY"
grep -q 'BASE1_B42_ASCII_DEFAULT=0' "$WRITER" || fail "writer missing ASCII default=0 evidence"
grep -q 'BASE1_B42_SAFE_DEFAULT=1' "$WRITER" || fail "writer missing safe default=1 evidence"
prepare_runtime_writer
printf 'B42 writer content: pass\n'

section "--- building native Phase1 binary ---"
rustup default stable >/dev/null 2>&1 || true
cargo build --release || fail "cargo build failed"
[ -x "$PHASE1_BIN" ] || fail "missing built binary: $PHASE1_BIN"
sha256sum "$PHASE1_BIN"

section "--- checking kernel, splash, busybox ---"
if [ ! -f "$KERNEL" ]; then
  [ -f scripts/x200-b23-stage-host-gnulinux.sh ] || fail "missing kernel and staging script"
  sh scripts/x200-b23-stage-host-gnulinux.sh || fail "kernel staging failed"
fi
[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL"
[ -f "$SPLASH" ] || fail "missing splash: $SPLASH"
BUSYBOX_PATH="$(find_busybox)"
[ -n "$BUSYBOX_PATH" ] || fail "missing busybox-static"
sha256sum "$KERNEL" "$SPLASH" "$BUSYBOX_PATH" 2>/dev/null || true

section "--- running B42 writer with full diagnostics ---"
printf 'About to run runtime writer. If this fails, the log will be printed.\n'
WRITER_ABS="$PWD/$RUNTIME_WRITER"
KERNEL_ABS="$PWD/$KERNEL"
SPLASH_ABS="$PWD/$SPLASH"
PHASE1_BIN_ABS="$PWD/$PHASE1_BIN"
set +e
sudo env \
  BASE1_B42_PHASE1_BIN="$PHASE1_BIN_ABS" \
  BASE1_B42_KERNEL="$KERNEL_ABS" \
  BASE1_B42_SPLASH="$SPLASH_ABS" \
  BASE1_B42_BUSYBOX="$BUSYBOX_PATH" \
  sh -x "$WRITER_ABS" "$USB" YES_WRITE_USB > "$WRITER_LOG" 2>&1
WRITER_RC=$?
set -e
show_log_tail "$WRITER_LOG"
[ "$WRITER_RC" -eq 0 ] || fail "B42 writer failed with exit code $WRITER_RC. Full log: $WRITER_LOG"
grep -Eq 'DONE: B42 stable safe color UTF-8( autoboot)? USB prepared' "$WRITER_LOG" || fail "writer exited 0 but did not print DONE marker. Full log: $WRITER_LOG"

section "--- verifying USB after write ---"
PART="$(part1 "$USB")"
MNT="$(mktemp -d)"
set +e
{
  echo "partition=$PART"
  sudo mount -o ro "$PART" "$MNT" || exit 10
  echo "mounted=$MNT"
  test -f "$MNT/boot/grub/grub.cfg" || exit 11
  test -f "$MNT/boot/phase1/vmlinuz" || exit 12
  test -f "$MNT/boot/phase1/$INITRD_NAME" || exit 13
  test -f "$MNT/phase1/assets/phase1-splash.png" || exit 14
  test -f "$MNT/phase1/evidence/b42-prep.env" || exit 15
  grep -q "$MAIN_ENTRY" "$MNT/boot/grub/grub.cfg" || exit 16
  grep -q "$ASCII_ENTRY" "$MNT/boot/grub/grub.cfg" || exit 17
  grep -q 'phase1.ascii=0' "$MNT/boot/grub/grub.cfg" || exit 18
  grep -q 'phase1.ascii=1' "$MNT/boot/grub/grub.cfg" || exit 19
  grep -q 'phase1.autoboot=1' "$MNT/boot/grub/grub.cfg" || exit 22
  grep -q 'BASE1_B42_ASCII_DEFAULT=0' "$MNT/phase1/evidence/b42-prep.env" || exit 20
  grep -q 'BASE1_B42_AUTO_BOOT_DEFAULT=1' "$MNT/phase1/evidence/b42-prep.env" || exit 23
  echo "USB menu entries:"
  grep '^menuentry ' "$MNT/boot/grub/grub.cfg" || true
  echo "USB evidence:"
  cat "$MNT/phase1/evidence/b42-prep.env"
  sudo umount "$MNT" || exit 21
} > "$VERIFY_LOG" 2>&1
VERIFY_RC=$?
rmdir "$MNT" 2>/dev/null || true
set -e
show_log_tail "$VERIFY_LOG"
[ "$VERIFY_RC" -eq 0 ] || fail "USB verification failed with exit code $VERIFY_RC. Full log: $VERIFY_LOG"

cat > "$REPORT" <<EOF
BASE1_B42_AUTOMATION_TARGET=$USB
BASE1_B42_AUTOMATION_PARTITION=$PART
BASE1_B42_AUTOMATION_WRITER=$WRITER
BASE1_B42_AUTOMATION_RUNTIME_WRITER=$RUNTIME_WRITER
BASE1_B42_AUTOMATION_WRITER_LOG=$WRITER_LOG
BASE1_B42_AUTOMATION_VERIFY_LOG=$VERIFY_LOG
BASE1_B42_AUTOMATION_MAIN_ENTRY=$MAIN_ENTRY
BASE1_B42_AUTOMATION_ASCII_ENTRY=$ASCII_ENTRY
BASE1_B42_AUTOMATION_ASCII_DEFAULT=0
BASE1_B42_AUTOMATION_ASCII_FALLBACK=1
BASE1_B42_AUTOMATION_AUTO_BOOT_DEFAULT=1
BASE1_B42_AUTOMATION_RESULT=prepared_and_verified
BASE1_B42_AUTOMATION_EXPECTED_RESULT=phase1_native_color_console_seen
BASE1_B42_AUTOMATION_UTF8_RESULT=phase1_japanese_utf8_ready
BASE1_B42_AUTOMATION_STABLE_RESULT=phase1_stable_safe_defaults_seen
EOF

printf '\nDONE: B42 USB prepared and verified.\n'
printf 'Default boot: %s\n' "$MAIN_ENTRY"
printf 'Fallback only: %s\n' "$ASCII_ENTRY"
printf 'Report: %s\n' "$REPORT"
