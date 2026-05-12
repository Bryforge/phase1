#!/usr/bin/env sh
# Phase1 / Base1 X200 B38 working Phase1 system USB writer.
#
# Purpose:
#   Freeze the first working Linux handoff from B37 and fix the runtime console.
#
# B37 learned:
#   A reboots, C reboots, E reboots, K reboots.
#   G boots far enough to reach Phase1-owned /init.
#
# B38 therefore uses B37 G as the default:
#   normal GRUB linux + normal initrd + rdinit=/init
#
# B38 also suppresses kernel log spam after /init starts so the phase1> prompt
# remains visible and interactive.
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B38_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B38_OUT:-build/base1-b38-working-phase1-system}"
KERNEL="${BASE1_B38_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
SPLASH_SRC="${BASE1_B38_SPLASH:-assets/phase1-splash.png}"
BUSYBOX="${BASE1_B38_BUSYBOX:-}"
REPORT="$OUT_DIR/b38-working-phase1-system-usb.env"
ROOTFS="$OUT_DIR/rootfs"
INITRD="$OUT_DIR/phase1-b38-system-initramfs.img"

fail() { printf 'x200-b38-working-phase1-system-usb: %s\n' "$1" >&2; exit 1; }
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

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b38-working-phase1-system-usb.sh /dev/sdb YES_WRITE_USB"
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
[ -n "$BUSYBOX_PATH" ] || fail "missing busybox; install busybox-static or set BASE1_B38_BUSYBOX=/path/to/static/busybox"
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

# Keep early setup quiet and make the console deterministic.
mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || mount -t sysfs sys /sys 2>/dev/null || true
mount -t devtmpfs devtmpfs /dev 2>/dev/null || mount -t tmpfs dev /dev 2>/dev/null || true
[ -c /dev/console ] || mknod /dev/console c 5 1 2>/dev/null || true
[ -c /dev/null ] || mknod /dev/null c 1 3 2>/dev/null || true
[ -c /dev/tty0 ] || mknod /dev/tty0 c 4 0 2>/dev/null || true
mount -t tmpfs tmpfs /run 2>/dev/null || true
mount -t tmpfs tmpfs /tmp 2>/dev/null || true
mkdir -p /phase1/evidence /phase1/state /phase1/workspace /phase1/help /phase1/bin

# Suppress asynchronous kernel log spam after the banner so phase1> remains visible.
echo 0 > /proc/sys/kernel/printk 2>/dev/null || true
if command -v dmesg >/dev/null 2>&1; then
  dmesg -n 1 2>/dev/null || true
fi

exec </dev/console >/dev/console 2>&1
stty sane 2>/dev/null || true

cat > /phase1/evidence/b38-working-system.env <<'ENV'
BASE1_B38_WORKING_SYSTEM_RESULT=phase1_full_system_load_seen
BASE1_B38_WORKING_PROTOCOL=normal_linux_normal_initrd_rdinit
BASE1_B38_SOURCE=B37_G_protocol
BASE1_B38_RUNTIME=phase1_owned_busybox_initramfs
BASE1_B38_CONSOLE_POLICY=kernel_log_suppressed_interactive_prompt
ENV

phase1_banner() {
  clear 2>/dev/null || true
  cat <<'BANNER'
phase1 6.0.0 ready
Base1 B38 working Phase1 system
result: phase1_full_system_load_seen

Working protocol: normal linux + normal initrd + rdinit=/init
Runtime: Phase1-owned BusyBox initramfs
Console: kernel log spam suppressed

Type: help, status, evidence, shell, dmesg, reboot, poweroff
BANNER
}

phase1_help() {
  cat <<'HELP'
commands:
  help       show commands
  status     show runtime status
  evidence   print evidence file
  shell      open /bin/sh
  dmesg      show recent kernel messages
  clear      redraw banner
  reboot     reboot
  poweroff   power off
HELP
}

phase1_status() {
  echo "runtime: Phase1-owned BusyBox initramfs"
  echo "kernel : $(uname -r 2>/dev/null || echo unknown)"
  echo "machine: $(uname -m 2>/dev/null || echo unknown)"
  echo "work   : /phase1/workspace"
  echo "evidence: /phase1/evidence"
}

phase1_banner
while true; do
  printf '\nphase1> '
  read cmd || cmd=shell
  case "$cmd" in
    help|h) phase1_help ;;
    status|s) phase1_status ;;
    evidence|e) cat /phase1/evidence/b38-working-system.env ;;
    shell|sh) /bin/sh ;;
    dmesg) echo 7 > /proc/sys/kernel/printk 2>/dev/null || true; dmesg | tail -n 80; echo 0 > /proc/sys/kernel/printk 2>/dev/null || true ;;
    clear|banner) phase1_banner ;;
    reboot) echo b > /proc/sysrq-trigger 2>/dev/null || reboot -f ;;
    poweroff|halt) poweroff -f 2>/dev/null || halt -f ;;
    "") : ;;
    *) echo "unknown command: $cmd"; echo "type: help" ;;
  esac
done
EOF
chmod 0755 "$ROOTFS/init"

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-b38-system-initramfs.img" )
[ -s "$INITRD" ] || fail "failed to build initramfs: $INITRD"

printf 'PHASE1 BASE1 B38 WORKING PHASE1 SYSTEM USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initramfs   : %s\n' "$INITRD"
printf 'splash      : %s\n' "$SPLASH_SRC"
printf 'busybox     : %s\n\n' "$BUSYBOX_PATH"
printf 'Kernel SHA256:\n'; sha256sum "$KERNEL"
printf 'B38 initramfs SHA256:\n'; sha256sum "$INITRD"
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
sudo mkfs.vfat -F 32 -n PHASE1B38 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/assets" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/phase1-b38-system-initramfs.img"
sudo cp "$SPLASH_SRC" "$MNT/phase1/assets/phase1-splash.png"
KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"
sudo tee "$MNT/phase1/evidence/b38-prep.env" >/dev/null <<EOF
BASE1_B38_PROFILE=$PROFILE
BASE1_B38_KERNEL=/boot/phase1/vmlinuz
BASE1_B38_INITRD=/boot/phase1/phase1-b38-system-initramfs.img
BASE1_B38_SPLASH=/phase1/assets/phase1-splash.png
BASE1_B38_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B38_INITRD_SIZE=$INITRD_SIZE
BASE1_B38_WORKING_PROTOCOL=normal_linux_normal_initrd_rdinit
BASE1_B38_EXPECTED_RESULT=phase1_full_system_load_seen
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=5
set default=1
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B38 Real Phase1 Splash" {
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
        echo "B38 real Phase1 splash active"
        sleep 3
    else
        terminal_output console
        clear
        echo "Real PNG splash unavailable."
        sleep 2
    fi
    terminal_output console
}

menuentry "Start Phase1 System" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B38 working Phase1 system"
    echo "Protocol: normal linux + normal initrd + rdinit=/init"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset quiet loglevel=0 panic=0
    initrd /boot/phase1/phase1-b38-system-initramfs.img
    boot
}

menuentry "Start Phase1 System - verbose" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B38 verbose working protocol"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=0
    initrd /boot/phase1/phase1-b38-system-initramfs.img
    boot
}

menuentry "B38 File check" {
    clear
    echo "B38 file check"
    ls -lh /boot/phase1
    echo ""
    cat /phase1/evidence/b38-prep.env
    echo ""
    sleep --interruptible 999
}

menuentry "B38 Command check" {
    clear
    echo "B38 command check: normal linux + normal initrd only"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=0
    echo "linux returned"
    initrd /boot/phase1/phase1-b38-system-initramfs.img
    echo "initrd returned"
    echo "No boot performed. ESC returns."
    sleep --interruptible 999
}

menuentry "B38 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B38 GRUB fallback console"
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
BASE1_B38_TARGET=$USB
BASE1_B38_PARTITION=$PART1
BASE1_B38_KERNEL=$KERNEL
BASE1_B38_INITRD=$INITRD
BASE1_B38_SPLASH=$SPLASH_SRC
BASE1_B38_RESULT=prepared
BASE1_B38_EXPECTED_NEXT_RESULT=phase1_full_system_load_seen
BASE1_B38_WORKING_PROTOCOL=normal_linux_normal_initrd_rdinit
EOF
printf '\nDONE: B38 working Phase1 system USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> external USB GRUB. Default entry starts Phase1.\n'
