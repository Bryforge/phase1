#!/usr/bin/env sh
# Phase1 / Base1 X200 B40 native Phase1 boot automation.
#
# Automates the current X200 native Phase1 boot preparation path:
#   1. update repo;
#   2. ensure/stage Linux-libre baseline kernel artifact;
#   3. build or use the Phase1 Rust binary;
#   4. check BusyBox/static runtime requirement;
#   5. write the B40 native-loader-fix USB using the proven B38 protocol.
#
# Usage:
#   sh scripts/x200-b40-prepare-native-phase1-boot.sh /dev/sdb YES_WRITE_USB
#
# Optional:
#   BASE1_AUTO_INSTALL_PACKAGES=1   install missing cargo/rustc/busybox-static on apt systems
#   BASE1_SKIP_PULL=1               skip git pull
#   BASE1_SKIP_BUILD=1              skip cargo build --release
#   BASE1_B40_PHASE1_BIN=/path/bin  use explicit Phase1 binary
#   BASE1_B40_KERNEL=/path/vmlinuz  use explicit kernel
#   BASE1_B40_BUSYBOX=/path/busybox use explicit busybox

set -eu

# Native rustup installs Cargo here. Add it early so non-login shells can find it.
if [ -d "$HOME/.cargo/bin" ]; then
  PATH="$HOME/.cargo/bin:$PATH"
  export PATH
fi
if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1090
  . "$HOME/.cargo/env" 2>/dev/null || true
fi

USB="${1:-}"
CONFIRM="${2:-}"
AUTO_INSTALL="${BASE1_AUTO_INSTALL_PACKAGES:-0}"
SKIP_PULL="${BASE1_SKIP_PULL:-0}"
SKIP_BUILD="${BASE1_SKIP_BUILD:-0}"
KERNEL="${BASE1_B40_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
SPLASH="${BASE1_B40_SPLASH:-assets/phase1-splash.png}"
PHASE1_BIN="${BASE1_B40_PHASE1_BIN:-target/release/phase1}"
B40_WRITER="scripts/x200-b40-native-loader-fix-usb.sh"
STAGER="scripts/x200-b23-stage-host-gnulinux.sh"
REPORT_DIR="build/base1-b40-native-loader-fix"
REPORT="$REPORT_DIR/b40-prepare-native-phase1-boot.env"

fail() { printf 'x200-b40-prepare-native-phase1-boot: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
section() { printf '\n%s\n' "$1"; }

usage() {
  cat <<'EOF'
Usage:
  sh scripts/x200-b40-prepare-native-phase1-boot.sh /dev/sdb YES_WRITE_USB

Optional environment:
  BASE1_AUTO_INSTALL_PACKAGES=1   install missing cargo/rustc/busybox-static on apt systems
  BASE1_SKIP_PULL=1               skip git pull
  BASE1_SKIP_BUILD=1              skip cargo build --release
  BASE1_B40_PHASE1_BIN=/path/bin  use explicit Phase1 binary
  BASE1_B40_KERNEL=/path/vmlinuz  use explicit kernel
  BASE1_B40_BUSYBOX=/path/busybox use explicit busybox
EOF
}

apt_install_if_allowed() {
  packages="$1"
  if [ "$AUTO_INSTALL" != "1" ]; then
    return 1
  fi
  command -v apt-get >/dev/null 2>&1 || return 1
  printf 'Installing packages with apt: %s\n' "$packages"
  sudo apt-get update
  # shellcheck disable=SC2086
  sudo apt-get install -y $packages
}

ensure_cargo_if_needed() {
  if [ "$SKIP_BUILD" = "1" ]; then
    return 0
  fi
  if command -v cargo >/dev/null 2>&1; then
    printf 'cargo: %s\n' "$(command -v cargo)"
    return 0
  fi
  printf 'Cargo is missing from PATH. Checked native rustup path: %s/.cargo/bin\n' "$HOME"
  if apt_install_if_allowed "cargo rustc"; then
    command -v cargo >/dev/null 2>&1 && return 0
  fi
  fail "missing command: cargo. Try: export PATH=\"$HOME/.cargo/bin:\$PATH\" or source ~/.cargo/env; or set BASE1_B40_PHASE1_BIN and BASE1_SKIP_BUILD=1"
}

find_busybox() {
  if [ -n "${BASE1_B40_BUSYBOX:-}" ]; then
    printf '%s\n' "$BASE1_B40_BUSYBOX"
    return
  fi
  for candidate in /bin/busybox /usr/bin/busybox /bin/busybox.static /usr/bin/busybox.static; do
    if [ -x "$candidate" ]; then
      printf '%s\n' "$candidate"
      return
    fi
  done
  printf '\n'
}

busybox_is_static() {
  bb="$1"
  if command -v ldd >/dev/null 2>&1; then
    if ldd "$bb" 2>&1 | grep -qi 'not a dynamic executable\|statically linked'; then
      return 0
    fi
    return 1
  fi
  if command -v file >/dev/null 2>&1 && file "$bb" | grep -qi 'statically linked'; then
    return 0
  fi
  return 0
}

install_busybox_static_if_allowed() {
  apt_install_if_allowed "busybox-static"
}

[ -n "$USB" ] || { usage; fail "missing USB block device"; }
[ "$CONFIRM" = "YES_WRITE_USB" ] || { usage; fail "missing YES_WRITE_USB confirmation"; }
[ -d .git ] || fail "run this from the phase1 repository root"
[ -f "$B40_WRITER" ] || fail "missing $B40_WRITER; run git pull first"

for cmd in sh git sudo grep awk sha256sum ls; do
  need_cmd "$cmd"
done

mkdir -p "$REPORT_DIR"

printf 'PHASE1 B40 NATIVE BOOT AUTOMATION\n\n'
printf 'target usb : %s\n' "$USB"
printf 'kernel     : %s\n' "$KERNEL"
printf 'phase1 bin : %s\n' "$PHASE1_BIN"
printf 'writer     : %s\n' "$B40_WRITER"
printf 'PATH       : %s\n' "$PATH"

if [ "$SKIP_PULL" != "1" ]; then
  section "--- updating repository ---"
  git pull --ff-only origin edge/stable
else
  section "--- repository update skipped ---"
fi

section "--- checking splash asset ---"
[ -f "$SPLASH" ] || fail "missing splash asset: $SPLASH"
sha256sum "$SPLASH"

section "--- checking/staging kernel artifact ---"
if [ ! -f "$KERNEL" ]; then
  [ -f "$STAGER" ] || fail "kernel missing and stager missing: $STAGER"
  printf 'Kernel missing. Staging host GNU/Linux kernel/initrd artifacts...\n'
  sh "$STAGER"
fi
[ -f "$KERNEL" ] || fail "kernel still missing after staging: $KERNEL"
sha256sum "$KERNEL"

section "--- building/checking Phase1 release binary ---"
ensure_cargo_if_needed
if [ "$SKIP_BUILD" != "1" ]; then
  cargo build --release
else
  printf 'cargo build skipped by BASE1_SKIP_BUILD=1\n'
fi
[ -x "$PHASE1_BIN" ] || fail "missing executable Phase1 binary: $PHASE1_BIN. Build with cargo or set BASE1_B40_PHASE1_BIN=/path/to/phase1"
sha256sum "$PHASE1_BIN"

section "--- checking BusyBox runtime ---"
BUSYBOX_PATH="$(find_busybox)"
if [ -z "$BUSYBOX_PATH" ] || ! busybox_is_static "$BUSYBOX_PATH"; then
  printf 'Static BusyBox missing or dynamic.\n'
  if install_busybox_static_if_allowed; then
    BUSYBOX_PATH="$(find_busybox)"
  fi
fi
[ -n "$BUSYBOX_PATH" ] || fail "missing BusyBox; run: sudo apt install -y busybox-static"
busybox_is_static "$BUSYBOX_PATH" || fail "BusyBox appears dynamic; run: sudo apt install -y busybox-static"
printf 'busybox: %s\n' "$BUSYBOX_PATH"
sha256sum "$BUSYBOX_PATH" 2>/dev/null || true

section "--- writing B40 native Phase1 USB ---"
sudo env \
  BASE1_B40_PHASE1_BIN="$PWD/$PHASE1_BIN" \
  BASE1_B40_KERNEL="$PWD/$KERNEL" \
  BASE1_B40_SPLASH="$PWD/$SPLASH" \
  BASE1_B40_BUSYBOX="$BUSYBOX_PATH" \
  sh "$B40_WRITER" "$USB" YES_WRITE_USB

cat > "$REPORT" <<EOF
BASE1_B40_PREP_TARGET=$USB
BASE1_B40_PREP_KERNEL=$KERNEL
BASE1_B40_PREP_PHASE1_BIN=$PHASE1_BIN
BASE1_B40_PREP_SPLASH=$SPLASH
BASE1_B40_PREP_BUSYBOX=$BUSYBOX_PATH
BASE1_B40_PREP_WRITER=$B40_WRITER
BASE1_B40_PREP_RESULT=prepared
BASE1_B40_PREP_EXPECTED_RESULT=phase1_native_console_seen
BASE1_B40_PREP_FALLBACK_RESULT=phase1_full_system_load_seen
EOF

printf '\nDONE: B40 native Phase1 USB prepared.\n'
printf 'Boot path:\n'
printf '  Libreboot external USB GRUB -> Start Native Phase1 Console\n\n'
printf 'Success target:\n'
printf '  phase1_native_console_seen\n\n'
printf 'Fallback target if the native binary still returns:\n'
printf '  phase1_full_system_load_seen\n\n'
printf 'Report: %s\n' "$REPORT"
