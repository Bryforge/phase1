#!/usr/bin/env sh
# Phase1 / Base1 X200 B36 boot protocol matrix USB writer.
#
# Purpose:
#   B35 still resets after boot. That means the problem is likely the exact
#   Linux boot protocol or kernel command line. B36 builds one USB with many
#   controlled protocol variants around the same known-good pieces:
#   - Libreboot GRUB display path
#   - real Phase1 splash PNG
#   - Linux-libre baseline kernel
#   - Phase1-owned BusyBox initramfs
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B36_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B36_OUT:-build/base1-b36-boot-protocol-matrix}"
KERNEL="${BASE1_B36_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
SPLASH_SRC="${BASE1_B36_SPLASH:-assets/phase1-splash.png}"
BUSYBOX="${BASE1_B36_BUSYBOX:-}"
REPORT="$OUT_DIR/b36-boot-protocol-matrix-usb.env"
ROOTFS="$OUT_DIR/rootfs"
INITRD="$OUT_DIR/phase1-b36-matrix-initramfs.img"

fail() { printf 'x200-b36-boot-protocol-matrix-usb: %s\n' "$1" >&2; exit 1; }
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

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b36-boot-protocol-matrix-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing Linux-libre baseline kernel: $KERNEL"
[ -f "$SPLASH_SRC" ] || fail "missing real Phase1 splash asset: $SPLASH_SRC"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod ln date stat file grep awk wc; do
  need_cmd "$cmd"
done

BUSYBOX_PATH="$(find_busybox)"
[ -n "$BUSYBOX_PATH" ] || fail "missing busybox; install busybox-static or set BASE1_B36_BUSYBOX=/path/to/static/busybox"
[ -x "$BUSYBOX_PATH" ] || fail "busybox is not executable: $BUSYBOX_PATH"
if command -v ldd >/dev/null 2>&1; then
  if ldd "$BUSYBOX_PATH" 2>&1 | grep -qi 'not a dynamic executable\|statically linked'; then
    BUSYBOX_STATIC=yes
  else
    BUSYBOX_STATIC=no
  fi
else
  BUSYBOX_STATIC=unknown
fi
[ "$BUSYBOX_STATIC" != no ] || fail "busybox appears dynamic; install busybox-static and rerun"

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
rm -rf "$ROOTFS"
mkdir -p "$ROOTFS/bin" "$ROOTFS/sbin" "$ROOTFS/dev" "$ROOTFS/proc" "$ROOTFS/sys" "$ROOTFS/run" "$ROOTFS/tmp" "$ROOTFS/etc" "$ROOTFS/phase1/bin" "$ROOTFS/phase1/evidence" "$ROOTFS/phase1/state" "$ROOTFS/phase1/workspace" "$ROOTFS/phase1/assets" "$ROOTFS/phase1/help"
cp "$BUSYBOX_PATH" "$ROOTFS/bin/busybox"
chmod 0755 "$ROOTFS/bin/busybox"
for app in sh ash mount umount mkdir cat echo ls dmesg uname sleep reboot poweroff halt mknod stty clear printf grep awk wc find sort sha256sum head tail date ps free df sync; do
  ln -sf busybox "$ROOTFS/bin/$app" 2>/dev/null || true
done
ln -sf ../bin/busybox "$ROOTFS/sbin/reboot" 2>/dev/null || true
ln -sf ../bin/busybox "$ROOTFS/sbin/poweroff" 2>/dev/null || true
ln -sf ../bin/busybox "$ROOTFS/sbin/halt" 2>/dev/null || true
cp "$SPLASH_SRC" "$ROOTFS/phase1/assets/phase1-splash.png"

cat > "$ROOTFS/init" <<'EOF'
#!/bin/sh
PATH=/phase1/bin:/bin:/sbin:/usr/bin:/usr/sbin
export PATH
mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || mount -t sysfs sys /sys 2>/dev/null || true
mount -t devtmpfs devtmpfs /dev 2>/dev/null || mount -t tmpfs dev /dev 2>/dev/null || true
[ -c /dev/console ] || mknod /dev/console c 5 1 2>/dev/null || true
[ -c /dev/null ] || mknod /dev/null c 1 3 2>/dev/null || true
[ -c /dev/tty0 ] || mknod /dev/tty0 c 4 0 2>/dev/null || true
mount -t tmpfs tmpfs /run 2>/dev/null || true
mount -t tmpfs tmpfs /tmp 2>/dev/null || true
mkdir -p /phase1/evidence /phase1/state /phase1/workspace /phase1/help /phase1/bin
exec </dev/console >/dev/console 2>&1
stty sane 2>/dev/null || true
cat > /phase1/evidence/b36-protocol.env <<'ENV'
BASE1_B36_BOOT_PROTOCOL_RESULT=phase1_full_system_load_seen
BASE1_B36_BOOT_PROTOCOL_MODE=phase1-owned-busybox-initramfs
ENV
clear 2>/dev/null || true
cat <<'BANNER'
phase1 6.0.0 ready
Base1 B36 boot protocol matrix
result: phase1_full_system_load_seen

Phase1-owned BusyBox runtime reached /init.
Type: help, status, shell, reboot, poweroff
BANNER
while true; do
  printf 'phase1> '
  read cmd || cmd=shell
  case "$cmd" in
    help|h) echo "commands: help status evidence shell dmesg reboot poweroff" ;;
    status|s) uname -a; echo "workspace: /phase1/workspace"; echo "evidence: /phase1/evidence" ;;
    evidence|e) cat /phase1/evidence/b36-protocol.env ;;
    dmesg) dmesg | tail -n 80 ;;
    shell|sh) /bin/sh ;;
    reboot) echo b > /proc/sysrq-trigger 2>/dev/null || reboot -f ;;
    poweroff|halt) poweroff -f 2>/dev/null || halt -f ;;
    "") : ;;
    *) echo "unknown command: $cmd" ;;
  esac
done
EOF
chmod 0755 "$ROOTFS/init"

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-b36-matrix-initramfs.img" )
[ -s "$INITRD" ] || fail "failed to build initramfs: $INITRD"

printf 'PHASE1 BASE1 B36 BOOT PROTOCOL MATRIX USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initramfs   : %s\n' "$INITRD"
printf 'splash      : %s\n' "$SPLASH_SRC"
printf 'busybox     : %s\n\n' "$BUSYBOX_PATH"
printf 'Kernel SHA256:\n'; sha256sum "$KERNEL"
printf 'B36 initramfs SHA256:\n'; sha256sum "$INITRD"
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
sudo mkfs.vfat -F 32 -n PHASE1B36 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/assets" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/phase1-b36-matrix-initramfs.img"
sudo cp "$SPLASH_SRC" "$MNT/phase1/assets/phase1-splash.png"
KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"
sudo tee "$MNT/phase1/evidence/b36-prep.env" >/dev/null <<EOF
BASE1_B36_PROFILE=$PROFILE
BASE1_B36_KERNEL=/boot/phase1/vmlinuz
BASE1_B36_INITRD=/boot/phase1/phase1-b36-matrix-initramfs.img
BASE1_B36_SPLASH=/phase1/assets/phase1-splash.png
BASE1_B36_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B36_INITRD_SIZE=$INITRD_SIZE
BASE1_B36_EXPECTED_RESULT=phase1_full_system_load_seen
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B36 Real Phase1 Splash" {
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
        echo "B36 real Phase1 splash active"
        echo "result: phase1_real_splash_seen"
        sleep --interruptible 999
    else
        terminal_output console
        clear
        echo "Real PNG splash unavailable."
        sleep --interruptible 999
    fi
    terminal_output console
}

menuentry "B36 File check" {
    clear
    echo "B36 file check"
    ls -lh /boot/phase1
    echo ""
    cat /phase1/evidence/b36-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B36 Test commands only" {
    clear
    echo "B36 test: linux16 + initrd16 command only"
    linux16 /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=0
    echo "linux16 returned"
    initrd16 /boot/phase1/phase1-b36-matrix-initramfs.img
    echo "initrd16 returned"
    sleep --interruptible 999
}

menuentry "B36 A linux16 rdinit no root" {
    clear
    echo "B36 A: linux16 rdinit no root"
    linux16 /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=0
    initrd16 /boot/phase1/phase1-b36-matrix-initramfs.img
    boot
}

menuentry "B36 B linux16 root ram0" {
    clear
    echo "B36 B: linux16 root ram0"
    linux16 /boot/phase1/vmlinuz root=/dev/ram0 ro console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=0
    initrd16 /boot/phase1/phase1-b36-matrix-initramfs.img
    boot
}

menuentry "B36 C linux16 no nomodeset" {
    clear
    echo "B36 C: linux16 no nomodeset"
    linux16 /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init loglevel=7 panic=0
    initrd16 /boot/phase1/phase1-b36-matrix-initramfs.img
    boot
}

menuentry "B36 D linux16 text vga normal" {
    clear
    echo "B36 D: linux16 text vga normal"
    linux16 /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init vga=normal nofb nomodeset loglevel=7 panic=0
    initrd16 /boot/phase1/phase1-b36-matrix-initramfs.img
    boot
}

menuentry "B36 E linux16 noapic acpi off" {
    clear
    echo "B36 E: linux16 noapic acpi off"
    linux16 /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset noapic nolapic acpi=off loglevel=7 panic=0
    initrd16 /boot/phase1/phase1-b36-matrix-initramfs.img
    boot
}

menuentry "B36 F normal linux rdinit" {
    clear
    echo "B36 F: normal linux rdinit"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=0
    initrd /boot/phase1/phase1-b36-matrix-initramfs.img
    boot
}

menuentry "B36 G normal linux text vga" {
    clear
    echo "B36 G: normal linux text vga"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init vga=normal nofb nomodeset loglevel=7 panic=0
    initrd /boot/phase1/phase1-b36-matrix-initramfs.img
    boot
}

menuentry "B36 H linux16 console ttyS0 also" {
    clear
    echo "B36 H: linux16 tty0 + ttyS0"
    linux16 /boot/phase1/vmlinuz console=tty0 console=ttyS0,115200n8 rdinit=/init init=/init nomodeset loglevel=7 panic=0
    initrd16 /boot/phase1/phase1-b36-matrix-initramfs.img
    boot
}

menuentry "B36 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B36 GRUB fallback console"
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
BASE1_B36_TARGET=$USB
BASE1_B36_PARTITION=$PART1
BASE1_B36_KERNEL=$KERNEL
BASE1_B36_INITRD=$INITRD
BASE1_B36_SPLASH=$SPLASH_SRC
BASE1_B36_RESULT=prepared
BASE1_B36_EXPECTED_NEXT_RESULT=phase1_full_system_load_seen
EOF
printf '\nDONE: B36 boot protocol matrix USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> external USB GRUB. Try B36 A first.\n'
