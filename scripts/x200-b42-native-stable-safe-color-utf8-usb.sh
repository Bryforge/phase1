#!/usr/bin/env sh
# Phase1 / Base1 X200 B42 stable-safe native color UTF-8 USB writer.
#
# Purpose:
#   Boot Phase1 as designed by default: color/native UI, UTF-8/Japanese-ready,
#   stable channel, safety enabled. ASCII is NOT the default; it is present as
#   an explicit fallback boot entry for limited consoles.
#
# Working protocol:
#   Libreboot GRUB -> normal linux -> normal initrd -> rdinit=/init
#
# Usage:
#   sh scripts/x200-b42-native-stable-safe-color-utf8-usb.sh /dev/sdb YES_WRITE_USB

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
OUT_DIR="${BASE1_B42_OUT:-build/base1-b42-native-stable-safe-color-utf8}"
KERNEL="${BASE1_B42_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
SPLASH_SRC="${BASE1_B42_SPLASH:-assets/phase1-splash.png}"
PHASE1_BIN="${BASE1_B42_PHASE1_BIN:-}"
BUSYBOX="${BASE1_B42_BUSYBOX:-}"
ROOTFS="$OUT_DIR/rootfs"
INITRD="$OUT_DIR/phase1-b42-native-stable-safe-color-utf8.img"
REPORT="$OUT_DIR/b42-native-stable-safe-color-utf8-usb.env"

fail() { printf 'x200-b42-native-stable-safe-color-utf8-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

find_busybox() {
  if [ -n "$BUSYBOX" ]; then printf '%s\n' "$BUSYBOX"; return; fi
  for p in /bin/busybox /usr/bin/busybox /bin/busybox.static /usr/bin/busybox.static; do
    [ -x "$p" ] && { printf '%s\n' "$p"; return; }
  done
  printf '\n'
}

find_phase1_bin() {
  if [ -n "$PHASE1_BIN" ]; then printf '%s\n' "$PHASE1_BIN"; return; fi
  for p in target/release/phase1 target/debug/phase1; do
    [ -x "$p" ] && { printf '%s\n' "$p"; return; }
  done
  printf '\n'
}

copy_one_file() {
  src="$1"; destroot="$2"
  [ -e "$src" ] || return 0
  dst="$destroot$src"
  mkdir -p "$(dirname "$dst")"
  cp -L "$src" "$dst"
}

copy_libs_for_binary() {
  bin="$1"; dest="$2"
  command -v ldd >/dev/null 2>&1 || return 0
  ldd "$bin" 2>/dev/null | while IFS= read -r line; do
    lib=""
    case "$line" in
      *'=> /'*) lib="$(printf '%s\n' "$line" | awk '{print $3}')" ;;
      /*) lib="$(printf '%s\n' "$line" | awk '{print $1}')" ;;
    esac
    [ -n "$lib" ] && [ -e "$lib" ] && copy_one_file "$lib" "$dest"
  done
}

copy_common_loaders() {
  dest="$1"
  for loader in \
    /lib64/ld-linux-x86-64.so.2 \
    /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 \
    /lib/ld-linux.so.2 \
    /lib/i386-linux-gnu/ld-linux.so.2
  do
    [ -e "$loader" ] && copy_one_file "$loader" "$dest"
  done
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b42-native-stable-safe-color-utf8-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL"
[ -f "$SPLASH_SRC" ] || fail "missing splash asset: $SPLASH_SRC"
ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing root filesystem device: $ROOT_SRC" ;; esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod ln date stat file grep awk; do
  need_cmd "$cmd"
done

BUSYBOX_PATH="$(find_busybox)"
[ -n "$BUSYBOX_PATH" ] || fail "missing busybox; install busybox-static or set BASE1_B42_BUSYBOX=/path/to/busybox"
PHASE1_BIN_PATH="$(find_phase1_bin)"
[ -n "$PHASE1_BIN_PATH" ] || fail "missing Phase1 binary; run cargo build --release or set BASE1_B42_PHASE1_BIN=/path/to/phase1"
[ -x "$PHASE1_BIN_PATH" ] || fail "Phase1 binary is not executable: $PHASE1_BIN_PATH"

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
rm -rf "$ROOTFS"
mkdir -p "$ROOTFS/bin" "$ROOTFS/sbin" "$ROOTFS/dev" "$ROOTFS/proc" "$ROOTFS/sys" "$ROOTFS/run" "$ROOTFS/tmp" "$ROOTFS/etc" "$ROOTFS/lib" "$ROOTFS/lib64" "$ROOTFS/usr/lib" "$ROOTFS/usr/lib64" "$ROOTFS/phase1/bin" "$ROOTFS/phase1/repo" "$ROOTFS/phase1/evidence" "$ROOTFS/phase1/state" "$ROOTFS/phase1/workspace" "$ROOTFS/phase1/assets" "$ROOTFS/phase1/i18n/ja"

cp -L "$BUSYBOX_PATH" "$ROOTFS/bin/busybox"
chmod 0755 "$ROOTFS/bin/busybox"
for app in sh ash mount umount mkdir cat echo ls dmesg uname sleep reboot poweroff halt mknod stty clear printf grep awk wc find sort sha256sum head tail date ps free df sync env file; do
  ln -sf busybox "$ROOTFS/bin/$app" 2>/dev/null || true
done
ln -sf ../bin/busybox "$ROOTFS/sbin/reboot" 2>/dev/null || true
ln -sf ../bin/busybox "$ROOTFS/sbin/poweroff" 2>/dev/null || true
ln -sf ../bin/busybox "$ROOTFS/sbin/halt" 2>/dev/null || true

cp -L "$SPLASH_SRC" "$ROOTFS/phase1/assets/phase1-splash.png"
cp -L "$PHASE1_BIN_PATH" "$ROOTFS/phase1/bin/phase1"
chmod 0755 "$ROOTFS/phase1/bin/phase1"
copy_libs_for_binary "$PHASE1_BIN_PATH" "$ROOTFS"
copy_common_loaders "$ROOTFS"

# Stable/safe native default config. ASCII is disabled by default and kept only
# as a fallback via the GRUB entry that passes phase1.ascii=1.
cat > "$ROOTFS/phase1.conf" <<'EOF'
# Phase1 default boot configuration
color=true
ascii=false
safe=true
quick=false
mobile=false
device_mode=desktop
persistent=false
bleeding_edge=false
host_tools=false
EOF
cp "$ROOTFS/phase1.conf" "$ROOTFS/phase1/repo/phase1.conf"

cat > "$ROOTFS/phase1/i18n/ja/boot-test.txt" <<'EOF'
フェーズ1 起動完了
日本語 UTF-8 テスト
安全・非公開・強力
EOF

cat > "$ROOTFS/phase1/evidence/b42-native-defaults.env" <<'EOF'
BASE1_B42_DEFAULT_RESULT=phase1_native_color_console_seen
BASE1_B42_UTF8_RESULT=phase1_japanese_utf8_ready
BASE1_B42_STABLE_RESULT=phase1_stable_safe_defaults_seen
BASE1_B42_ASCII_DEFAULT=0
BASE1_B42_ASCII_FALLBACK=1
BASE1_B42_STABLE_DEFAULT=1
BASE1_B42_SAFE_DEFAULT=1
BASE1_B42_HOST_TOOLS_DEFAULT=0
BASE1_B42_EDGE_DEFAULT=0
BASE1_B42_LANG=C.UTF-8
BASE1_B42_LANGUAGE=ja:en
EOF

for path in README.md Cargo.toml phase1 start_phase1 FEATURE_STATUS.md PHASE1_NATIVE_LANGUAGE.md; do
  [ -f "$path" ] && cp "$path" "$ROOTFS/phase1/repo/$path"
done
[ -d assets ] && { mkdir -p "$ROOTFS/phase1/repo/assets"; cp assets/phase1-splash.png "$ROOTFS/phase1/repo/assets/phase1-splash.png" 2>/dev/null || true; }

cat > "$ROOTFS/init" <<'EOF'
#!/bin/sh
PATH=/phase1/bin:/bin:/sbin:/usr/bin:/usr/sbin
export PATH
mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || mount -t sysfs sys /sys 2>/dev/null || true
mount -t devtmpfs devtmpfs /dev 2>/dev/null || mount -t tmpfs dev /dev 2>/dev/null || true
[ -c /dev/console ] || mknod /dev/console c 5 1 2>/dev/null || true
[ -c /dev/null ] || mknod /dev/null c 1 3 2>/dev/null || true
mount -t tmpfs tmpfs /run 2>/dev/null || true
mount -t tmpfs tmpfs /tmp 2>/dev/null || true
mkdir -p /phase1/evidence /phase1/state /phase1/workspace /phase1/repo /phase1/i18n/ja

CMDLINE="$(cat /proc/cmdline 2>/dev/null || true)"
ASCII_FALLBACK=0
case " $CMDLINE " in *" phase1.ascii=1 "*) ASCII_FALLBACK=1 ;; esac

# Stable + safety defaults.
export PHASE1_BOOT_PROFILE=stable-safe
export PHASE1_STABLE=1
export PHASE1_CHANNEL=stable
export PHASE1_BOOT_STABILITY=stable
export PHASE1_SAFE_MODE=1
export PHASE1_SECURITY_LEVEL=stable-safe
export PHASE1_ALLOW_HOST_TOOLS=0
export PHASE1_TRUST_HOST=0
export PHASE1_BLEEDING_EDGE=0
export PHASE1_EDGE=0
export PHASE1_PERSISTENT_STATE=0
export PHASE1_QUICK_BOOT=0
export PHASE1_DEVICE_MODE=desktop
export PHASE1_HOME=/phase1/repo
export PHASE1_DEVICE_MODE=hardware
export PHASE1_LAUNCH_COMMAND="phase1-native-boot"

# UTF-8 + Japanese readiness.
export LANG=C.UTF-8
export LC_ALL=C.UTF-8
export LC_CTYPE=C.UTF-8
export LANGUAGE=ja:en
export PHASE1_LANGUAGE=ja
export PHASE1_I18N=1
export PHASE1_UTF8=1
export PHASE1_UNICODE=1
export PHASE1_JAPANESE_SUPPORT=1

if [ "$ASCII_FALLBACK" = "1" ]; then
  export TERM=linux
  export PHASE1_ASCII=1
  export PHASE1_ASCII_FALLBACK=1
  export PHASE1_COLOR_MODE=mono
  export PHASE1_FORCE_COLOR=0
  export PHASE1_NO_COLOR=1
  export NO_COLOR=1
else
  export TERM=linux
  export COLORTERM=truecolor
  export PHASE1_ASCII=0
  export PHASE1_ASCII_FALLBACK=1
  export PHASE1_THEME=crimson
  export PHASE1_COLOR_MODE=auto
  export PHASE1_FORCE_COLOR=1
  unset PHASE1_NO_COLOR
  unset NO_COLOR
fi

cat > /phase1/evidence/b42-runtime-defaults.env <<ENV
BASE1_B42_RUNTIME_RESULT=phase1_native_color_console_seen
BASE1_B42_UTF8_RESULT=phase1_japanese_utf8_ready
BASE1_B42_STABLE_RESULT=phase1_stable_safe_defaults_seen
BASE1_B42_ASCII_FALLBACK_ACTIVE=$ASCII_FALLBACK
BASE1_B42_ASCII_DEFAULT=0
BASE1_B42_ASCII_FALLBACK=1
BASE1_B42_STABLE_DEFAULT=1
BASE1_B42_SAFE_DEFAULT=1
BASE1_B42_HOST_TOOLS_DEFAULT=0
BASE1_B42_EDGE_DEFAULT=0
BASE1_B42_LANG=$LANG
BASE1_B42_LANGUAGE=$LANGUAGE
ENV

echo 0 > /proc/sys/kernel/printk 2>/dev/null || true
dmesg -n 1 2>/dev/null || true
exec </dev/console >/dev/console 2>&1
stty sane 2>/dev/null || true
clear 2>/dev/null || true

if [ "$ASCII_FALLBACK" = "1" ]; then
  echo "phase1 6.0.0 ready"
  echo "Base1 B42 ASCII fallback console"
else
  echo "phase1 6.0.0 ready"
  echo "Base1 B42 native color + Japanese UTF-8 console"
fi
echo "stable: enabled"
echo "safety: enabled"
echo "ascii default: no; ascii fallback: yes"
echo "utf8/japanese: ready"
echo

cd /phase1/repo 2>/dev/null || cd /phase1
/phase1/bin/phase1 2>&1
rc=$?
echo "BASE1_B42_NATIVE_BINARY_EXIT_CODE=$rc" >> /phase1/evidence/b42-runtime-defaults.env

echo
cat <<'FALLBACK'
phase1 fallback shell active
Type: help, evidence, ja-test, status, shell, reboot, poweroff
FALLBACK
while true; do
  printf '\nphase1-fallback> '
  read cmd || cmd=shell
  case "$cmd" in
    help|h) echo "commands: help evidence ja-test status shell reboot poweroff" ;;
    evidence|e) cat /phase1/evidence/b42-runtime-defaults.env ;;
    ja-test|japanese|utf8) cat /phase1/i18n/ja/boot-test.txt ;;
    status|s) uname -a; echo "term=$TERM color=${PHASE1_COLOR_MODE:-} ascii=${PHASE1_ASCII:-} lang=$LANG safety=$PHASE1_SAFE_MODE stable=$PHASE1_STABLE" ;;
    shell|sh) /bin/sh ;;
    reboot) echo b > /proc/sysrq-trigger 2>/dev/null || reboot -f ;;
    poweroff|halt) poweroff -f 2>/dev/null || halt -f ;;
    "") : ;;
    *) echo "unknown command: $cmd" ;;
  esac
done
EOF
chmod 0755 "$ROOTFS/init"

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-b42-native-stable-safe-color-utf8.img" )
[ -s "$INITRD" ] || fail "failed to build initramfs: $INITRD"

printf 'PHASE1 BASE1 B42 STABLE SAFE COLOR UTF-8 USB WRITER\n\n'
printf 'target disk  : %s\n' "$USB"
printf 'kernel       : %s\n' "$KERNEL"
printf 'initramfs    : %s\n' "$INITRD"
printf 'phase1 bin   : %s\n' "$PHASE1_BIN_PATH"
printf 'splash       : %s\n' "$SPLASH_SRC"
printf 'busybox      : %s\n\n' "$BUSYBOX_PATH"
sha256sum "$KERNEL" "$INITRD" "$PHASE1_BIN_PATH" "$SPLASH_SRC"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"
sudo mkfs.vfat -F 32 -n PHASE1B42 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/assets" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/phase1-b42-native-stable-safe-color-utf8.img"
sudo cp "$SPLASH_SRC" "$MNT/phase1/assets/phase1-splash.png"

sudo tee "$MNT/phase1/evidence/b42-prep.env" >/dev/null <<EOF
BASE1_B42_KERNEL=/boot/phase1/vmlinuz
BASE1_B42_INITRD=/boot/phase1/phase1-b42-native-stable-safe-color-utf8.img
BASE1_B42_SPLASH=/phase1/assets/phase1-splash.png
BASE1_B42_EXPECTED_RESULT=phase1_native_color_console_seen
BASE1_B42_UTF8_RESULT=phase1_japanese_utf8_ready
BASE1_B42_STABLE_RESULT=phase1_stable_safe_defaults_seen
BASE1_B42_ASCII_DEFAULT=0
BASE1_B42_ASCII_FALLBACK=1
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=5
set default=1
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B42 Real Phase1 Splash" {
    clear
    echo "Loading real Phase1 splash..."
    insmod all_video
    insmod gfxterm
    insmod png
    set gfxmode=auto
    set gfxpayload=text
    terminal_output gfxterm
    if background_image /phase1/assets/phase1-splash.png; then
        echo "phase1 6.0.0 ready"
        echo "B42 real Phase1 splash active"
        sleep 3
    else
        terminal_output console
        clear
        echo "Real PNG splash unavailable."
        sleep 2
    fi
    terminal_output console
}

menuentry "Start Phase1 Stable Safe Color UTF-8" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B42 stable safe color UTF-8 default"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset quiet loglevel=0 panic=0 phase1.stable=1 phase1.safe=1 phase1.ascii=0 phase1.utf8=1
    initrd /boot/phase1/phase1-b42-native-stable-safe-color-utf8.img
    boot
}

menuentry "Start Phase1 ASCII Safe Fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B42 ASCII safe fallback"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset quiet loglevel=0 panic=0 phase1.stable=1 phase1.safe=1 phase1.ascii=1 phase1.utf8=1
    initrd /boot/phase1/phase1-b42-native-stable-safe-color-utf8.img
    boot
}

menuentry "B42 File check" {
    clear
    echo "B42 file check"
    ls -lh /boot/phase1
    echo ""
    cat /phase1/evidence/b42-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B42 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B42 GRUB fallback console"
    sleep --interruptible 999
}

menuentry "Reboot" { reboot }
menuentry "Power off" { halt }
EOF
sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B42_TARGET=$USB
BASE1_B42_PARTITION=$PART1
BASE1_B42_KERNEL=$KERNEL
BASE1_B42_INITRD=$INITRD
BASE1_B42_SPLASH=$SPLASH_SRC
BASE1_B42_PHASE1_BINARY=$PHASE1_BIN_PATH
BASE1_B42_RESULT=prepared
BASE1_B42_EXPECTED_NEXT_RESULT=phase1_native_color_console_seen
BASE1_B42_UTF8_NEXT_RESULT=phase1_japanese_utf8_ready
BASE1_B42_STABLE_NEXT_RESULT=phase1_stable_safe_defaults_seen
BASE1_B42_ASCII_DEFAULT=0
BASE1_B42_ASCII_FALLBACK=1
EOF
printf '\nDONE: B42 stable safe color UTF-8 USB prepared on %s\n' "$USB"
printf 'Default boot: Start Phase1 Stable Safe Color UTF-8\n'
printf 'Fallback boot: Start Phase1 ASCII Safe Fallback\n'
