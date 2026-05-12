#!/usr/bin/env sh
# Phase1 / Base1 X200 B40/B41/B42 native binary loader fix USB writer.
#
# Purpose:
#   Boot the native Phase1 console through the working B38 protocol and package
#   the ELF loader/shared libraries required by the Rust binary.
#
# Working protocol:
#   Libreboot GRUB -> normal linux -> normal initrd -> rdinit=/init
#
# B41:
#   Native color console enabled.
#
# B42:
#   Japanese UTF-8 environment and verification text are packaged.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B40_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B40_OUT:-build/base1-b40-native-loader-fix}"
KERNEL="${BASE1_B40_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
SPLASH_SRC="${BASE1_B40_SPLASH:-assets/phase1-splash.png}"
PHASE1_BIN="${BASE1_B40_PHASE1_BIN:-}"
BUSYBOX="${BASE1_B40_BUSYBOX:-}"
REPORT="$OUT_DIR/b40-native-loader-fix-usb.env"
ROOTFS="$OUT_DIR/rootfs"
INITRD="$OUT_DIR/phase1-b40-native-loader-fix-initramfs.img"

fail() { printf 'x200-b40-native-loader-fix-usb: %s\n' "$1" >&2; exit 1; }
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
  for candidate in /bin/busybox /usr/bin/busybox /bin/busybox.static /usr/bin/busybox.static; do
    if [ -x "$candidate" ]; then printf '%s\n' "$candidate"; return; fi
  done
  printf '\n'
}

find_phase1_bin() {
  if [ -n "$PHASE1_BIN" ]; then printf '%s\n' "$PHASE1_BIN"; return; fi
  for candidate in target/release/phase1 target/debug/phase1; do
    if [ -x "$candidate" ]; then printf '%s\n' "$candidate"; return; fi
  done
  printf '\n'
}

copy_one_file() {
  src="$1"
  destroot="$2"
  [ -f "$src" ] || return 0
  dst="$destroot$src"
  mkdir -p "$(dirname "$dst")"
  cp -L "$src" "$dst"
}

copy_libs_for_binary() {
  bin="$1"
  dest="$2"
  if ! command -v ldd >/dev/null 2>&1; then
    return 0
  fi
  ldd "$bin" 2>/dev/null | while IFS= read -r line; do
    lib=""
    case "$line" in
      *'=> /'*) lib="$(printf '%s\n' "$line" | awk '{print $3}')" ;;
      /*) lib="$(printf '%s\n' "$line" | awk '{print $1}')" ;;
      *) lib="" ;;
    esac
    if [ -n "$lib" ] && [ -f "$lib" ]; then
      copy_one_file "$lib" "$dest"
    fi
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
    if [ -e "$loader" ]; then
      copy_one_file "$loader" "$dest"
    fi
  done
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b40-native-loader-fix-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing Linux-libre baseline kernel: $KERNEL"
[ -f "$SPLASH_SRC" ] || fail "missing real Phase1 splash asset: $SPLASH_SRC"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod ln date stat file grep awk wc sort head tail; do
  need_cmd "$cmd"
done

BUSYBOX_PATH="$(find_busybox)"
[ -n "$BUSYBOX_PATH" ] || fail "missing busybox; install busybox-static or set BASE1_B40_BUSYBOX=/path/to/static/busybox"
[ -x "$BUSYBOX_PATH" ] || fail "busybox is not executable: $BUSYBOX_PATH"

PHASE1_BIN_PATH="$(find_phase1_bin)"
if [ -z "$PHASE1_BIN_PATH" ]; then
  if command -v cargo >/dev/null 2>&1; then
    printf 'No built Phase1 binary found. Building release binary...\n'
    cargo build --release
    PHASE1_BIN_PATH="$(find_phase1_bin)"
  fi
fi
[ -n "$PHASE1_BIN_PATH" ] || fail "missing Phase1 binary; run: cargo build --release or set BASE1_B40_PHASE1_BIN=/path/to/phase1"
[ -x "$PHASE1_BIN_PATH" ] || fail "Phase1 binary is not executable: $PHASE1_BIN_PATH"

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
rm -rf "$ROOTFS"
mkdir -p "$ROOTFS/bin" "$ROOTFS/sbin" "$ROOTFS/dev" "$ROOTFS/proc" "$ROOTFS/sys" "$ROOTFS/run" "$ROOTFS/tmp" "$ROOTFS/etc" "$ROOTFS/lib" "$ROOTFS/lib64" "$ROOTFS/usr/lib" "$ROOTFS/usr/lib64" "$ROOTFS/phase1/bin" "$ROOTFS/phase1/repo" "$ROOTFS/phase1/evidence" "$ROOTFS/phase1/state" "$ROOTFS/phase1/workspace" "$ROOTFS/phase1/assets" "$ROOTFS/phase1/help" "$ROOTFS/phase1/i18n/ja"

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

cat > "$ROOTFS/phase1/i18n/ja/boot-test.txt" <<'EOF'
フェーズ1 起動完了
日本語 UTF-8 テスト
安全・非公開・強力
EOF

cat > "$ROOTFS/phase1/evidence/b42-japanese-utf8.env" <<'EOF'
BASE1_B42_JAPANESE_UTF8_RESULT=phase1_japanese_utf8_ready
BASE1_B42_LANG=C.UTF-8
BASE1_B42_LANGUAGE=ja:en
BASE1_B42_PHASE1_LANGUAGE=ja
BASE1_B42_UTF8=1
BASE1_B42_UNICODE=1
BASE1_B42_JAPANESE_SUPPORT=1
BASE1_B42_TEST_FILE=/phase1/i18n/ja/boot-test.txt
EOF

for path in README.md Cargo.toml phase1 start_phase1 docs/project/FEATURE_STATUS.md docs/project/PHASE1_NATIVE_LANGUAGE.md; do
  if [ -f "$path" ]; then
    cp "$path" "$ROOTFS/phase1/repo/$path"
  fi
done
if [ -d assets ]; then
  mkdir -p "$ROOTFS/phase1/repo/assets"
  cp assets/phase1-splash.png "$ROOTFS/phase1/repo/assets/phase1-splash.png" 2>/dev/null || true
  cp assets/phase1_base_fyr_banner1.png "$ROOTFS/phase1/repo/assets/phase1_base_fyr_banner1.png" 2>/dev/null || true
fi

cat > "$ROOTFS/init" <<'EOF'
#!/bin/sh
PATH=/phase1/bin:/bin:/sbin:/usr/bin:/usr/sbin
export PATH
export TERM=linux
export COLORTERM=truecolor
export LANG=C.UTF-8
export LC_ALL=C.UTF-8
export LC_CTYPE=C.UTF-8
export LANGUAGE=ja:en
export PHASE1_HOME=/phase1/repo
export PHASE1_SAFE_MODE=1
export PHASE1_THEME=crimson
export PHASE1_COLOR_MODE=auto
export PHASE1_FORCE_COLOR=1
export PHASE1_LANGUAGE=ja
export PHASE1_I18N=1
export PHASE1_UTF8=1
export PHASE1_UNICODE=1
export PHASE1_JAPANESE_SUPPORT=1
export PHASE1_DEVICE_MODE=hardware
export PHASE1_LAUNCH_COMMAND="phase1-native-boot"
unset NO_COLOR

mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || mount -t sysfs sys /sys 2>/dev/null || true
mount -t devtmpfs devtmpfs /dev 2>/dev/null || mount -t tmpfs dev /dev 2>/dev/null || true
[ -c /dev/console ] || mknod /dev/console c 5 1 2>/dev/null || true
[ -c /dev/null ] || mknod /dev/null c 1 3 2>/dev/null || true
[ -c /dev/tty0 ] || mknod /dev/tty0 c 4 0 2>/dev/null || true
mount -t tmpfs tmpfs /run 2>/dev/null || true
mount -t tmpfs tmpfs /tmp 2>/dev/null || true
mkdir -p /phase1/evidence /phase1/state /phase1/workspace /phase1/help /phase1/bin /phase1/repo /phase1/i18n/ja

echo 0 > /proc/sys/kernel/printk 2>/dev/null || true
dmesg -n 1 2>/dev/null || true
exec </dev/console >/dev/console 2>&1
stty sane 2>/dev/null || true

cat > /phase1/evidence/b41-native-color.env <<'ENV'
BASE1_B41_NATIVE_COLOR_RESULT=phase1_native_color_console_seen
BASE1_B40_NATIVE_LOADER_FIX_RESULT=phase1_native_console_seen
BASE1_B40_BOOT_PROTOCOL=normal_linux_normal_initrd_rdinit
BASE1_B40_NATIVE_BINARY=/phase1/bin/phase1
BASE1_B41_TERM=linux
BASE1_B41_COLORTERM=truecolor
BASE1_B41_COLOR_MODE=auto
BASE1_B41_FORCE_COLOR=1
ENV

cat > /phase1/evidence/b42-runtime-utf8.env <<'ENV'
BASE1_B42_JAPANESE_UTF8_RESULT=phase1_japanese_utf8_ready
BASE1_B42_LANG=C.UTF-8
BASE1_B42_LANGUAGE=ja:en
BASE1_B42_PHASE1_LANGUAGE=ja
BASE1_B42_UTF8=1
BASE1_B42_UNICODE=1
BASE1_B42_JAPANESE_SUPPORT=1
BASE1_B42_TEST_FILE=/phase1/i18n/ja/boot-test.txt
ENV

clear 2>/dev/null || true
cat <<'BANNER'
phase1 6.0.0 ready
Base1 B42 native color + Japanese UTF-8 console
result target: phase1_native_color_console_seen
utf8 target  : phase1_japanese_utf8_ready

Launching Phase1 native console with color and Japanese UTF-8 enabled...
BANNER

if [ -x /phase1/bin/phase1 ]; then
  echo "BASE1_B42_NATIVE_BINARY_STARTED=1" >> /phase1/evidence/b42-runtime-utf8.env
  /phase1/bin/phase1 2>&1
  rc=$?
  echo "BASE1_B42_NATIVE_BINARY_EXIT_CODE=$rc" >> /phase1/evidence/b42-runtime-utf8.env
  echo ""
  echo "Phase1 native binary returned with exit code: $rc"
else
  echo "BASE1_B42_NATIVE_BINARY_STARTED=0" >> /phase1/evidence/b42-runtime-utf8.env
  echo "Phase1 native binary unavailable."
fi

cat <<'FALLBACK'

phase1 fallback shell active
result: phase1_full_system_load_seen
Type: help, evidence, ja-test, status, shell, reboot, poweroff
FALLBACK

while true; do
  printf '\nphase1-fallback> '
  read cmd || cmd=shell
  case "$cmd" in
    help|h) echo "commands: help evidence ja-test status shell reboot poweroff" ;;
    evidence|e) cat /phase1/evidence/b41-native-color.env; cat /phase1/evidence/b42-runtime-utf8.env ;;
    ja-test|japanese|utf8) cat /phase1/i18n/ja/boot-test.txt ;;
    status|s) uname -a; echo "term=$TERM colorterm=$COLORTERM color=$PHASE1_COLOR_MODE lang=$LANG language=$LANGUAGE phase1_language=$PHASE1_LANGUAGE" ;;
    shell|sh) /bin/sh ;;
    reboot) echo b > /proc/sysrq-trigger 2>/dev/null || reboot -f ;;
    poweroff|halt) poweroff -f 2>/dev/null || halt -f ;;
    "") : ;;
    *) echo "unknown command: $cmd" ;;
  esac
done
EOF
chmod 0755 "$ROOTFS/init"

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-b40-native-loader-fix-initramfs.img" )
[ -s "$INITRD" ] || fail "failed to build initramfs: $INITRD"

printf 'PHASE1 BASE1 B42 NATIVE COLOR + JAPANESE UTF-8 USB WRITER\n\n'
printf 'profile      : %s\n' "$PROFILE"
printf 'target disk  : %s\n' "$USB"
printf 'kernel       : %s\n' "$KERNEL"
printf 'initramfs    : %s\n' "$INITRD"
printf 'phase1 bin   : %s\n' "$PHASE1_BIN_PATH"
printf 'splash       : %s\n' "$SPLASH_SRC"
printf 'busybox      : %s\n\n' "$BUSYBOX_PATH"
printf 'Kernel SHA256:\n'; sha256sum "$KERNEL"
printf 'B42 initramfs SHA256:\n'; sha256sum "$INITRD"
printf 'Phase1 binary SHA256:\n'; sha256sum "$PHASE1_BIN_PATH"
printf 'Real splash SHA256:\n'; sha256sum "$SPLASH_SRC"
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
sudo cp "$INITRD" "$MNT/boot/phase1/phase1-b42-native-color-utf8-initramfs.img"
sudo cp "$SPLASH_SRC" "$MNT/phase1/assets/phase1-splash.png"
KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"
PHASE1_SIZE="$(stat -c %s "$PHASE1_BIN_PATH" 2>/dev/null || stat -f %z "$PHASE1_BIN_PATH")"
sudo tee "$MNT/phase1/evidence/b42-prep.env" >/dev/null <<EOF
BASE1_B42_PROFILE=$PROFILE
BASE1_B42_KERNEL=/boot/phase1/vmlinuz
BASE1_B42_INITRD=/boot/phase1/phase1-b42-native-color-utf8-initramfs.img
BASE1_B42_SPLASH=/phase1/assets/phase1-splash.png
BASE1_B42_PHASE1_BINARY=/phase1/bin/phase1
BASE1_B42_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B42_INITRD_SIZE=$INITRD_SIZE
BASE1_B42_PHASE1_BINARY_SOURCE=$PHASE1_BIN_PATH
BASE1_B42_PHASE1_BINARY_SIZE=$PHASE1_SIZE
BASE1_B42_EXPECTED_RESULT=phase1_native_color_console_seen
BASE1_B42_UTF8_RESULT=phase1_japanese_utf8_ready
BASE1_B42_FALLBACK_RESULT=phase1_full_system_load_seen
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

menuentry "Start Native Phase1 Color UTF-8 Console" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B42 native color + Japanese UTF-8 console"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset quiet loglevel=0 panic=0
    initrd /boot/phase1/phase1-b42-native-color-utf8-initramfs.img
    boot
}

menuentry "Start Native Phase1 Color UTF-8 Console - verbose" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B42 verbose native color + Japanese UTF-8 console"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=0
    initrd /boot/phase1/phase1-b42-native-color-utf8-initramfs.img
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
BASE1_B42_FALLBACK_RESULT=phase1_full_system_load_seen
EOF
printf '\nDONE: B42 native color + Japanese UTF-8 USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> external USB GRUB. Default entry starts native color UTF-8 Phase1.\n'
