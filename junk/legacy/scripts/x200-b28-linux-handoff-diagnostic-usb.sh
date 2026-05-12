#!/usr/bin/env sh
# Phase1 / Base1 X200 B28 Linux handoff diagnostic USB writer.
#
# Purpose:
#   Diagnose where the SeaBIOS USB GRUB -> GNU/Linux handoff is blocking.
#   B27 reached the GRUB echo before the linux command but did not visibly
#   enter Linux. B28 adds step-by-step GRUB entries that show whether the
#   linux command returns, whether initrd returns, and whether boot hands off.
#
# Route:
#   Libreboot -> SeaBIOS payload -> USB GRUB -> Linux handoff diagnostics
#
# Required local artifacts by default:
#   build/linux/alpine-netboot/vmlinuz
#   build/linux/alpine-netboot/initrd.img
#
# Safety:
#   This erases only the selected USB disk, refuses to run without
#   YES_WRITE_USB, and refuses /dev/sda by default.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B28_PROFILE:-x200-supervisor-concurrent-lab}"
OUT_DIR="${BASE1_B28_OUT:-build/base1-b28-linux-handoff-diagnostic}"
KERNEL="${BASE1_B28_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B28_INITRD:-build/linux/alpine-netboot/initrd.img}"
REPORT="$OUT_DIR/b28-linux-handoff-diagnostic-usb.env"
OVERLAY_DIR="$OUT_DIR/initramfs-overlay"
OVERLAY="$OUT_DIR/phase1-b28-overlay.cpio.gz"
COMBINED="$OUT_DIR/phase1-b28-combined-initrd.img"

fail() { printf 'x200-b28-linux-handoff-diagnostic-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b28-linux-handoff-diagnostic-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing GNU/Linux kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing GNU/Linux initrd: $INITRD"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write the root filesystem device: $ROOT_SRC" ;;
esac

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod cat date stat; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
MNT="$(mktemp -d)"

cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
rm -rf "$OVERLAY_DIR"
mkdir -p "$OVERLAY_DIR/dev" "$OVERLAY_DIR/proc" "$OVERLAY_DIR/sys" "$OVERLAY_DIR/tmp" "$OVERLAY_DIR/phase1/evidence" "$OVERLAY_DIR/phase1/workspace"

cat > "$OVERLAY_DIR/init" <<'EOF'
#!/bin/sh
PATH=/bin:/sbin:/usr/bin:/usr/sbin
export PATH
mount -t proc proc /proc 2>/dev/null || true
mount -t sysfs sysfs /sys 2>/dev/null || true
mount -t devtmpfs devtmpfs /dev 2>/dev/null || mount -t tmpfs dev /dev 2>/dev/null || true
[ -c /dev/console ] || mknod /dev/console c 5 1 2>/dev/null || true
exec </dev/console >/dev/console 2>&1
clear 2>/dev/null || true
mkdir -p /phase1/evidence /phase1/workspace
cat > /phase1/evidence/b28-runtime.env <<'ENV'
BASE1_B28_LINUX_HANDOFF_RESULT=phase1_b28_linux_init_seen
BASE1_B28_LINUX_HANDOFF_MODE=diagnostic-initramfs
ENV
cat <<'BANNER'
phase1 6.0.0 ready
Base1 B28 Linux handoff diagnostic
result: phase1_b28_linux_init_seen

Linux reached /init.
Type: shell, evidence, reboot, poweroff
BANNER
while true; do
  printf 'phase1-b28> '
  read cmd || cmd=shell
  case "$cmd" in
    shell|sh) /bin/sh ;;
    evidence|e) cat /phase1/evidence/b28-runtime.env ;;
    reboot) echo b > /proc/sysrq-trigger 2>/dev/null || reboot -f ;;
    poweroff|halt) poweroff -f 2>/dev/null || halt -f ;;
    "") : ;;
    *) echo "commands: shell evidence reboot poweroff" ;;
  esac
done
EOF
chmod 0755 "$OVERLAY_DIR/init"

printf 'PHASE1 BASE1 B28 LINUX HANDOFF DIAGNOSTIC USB WRITER\n\n'
printf 'profile     : %s\n' "$PROFILE"
printf 'target disk : %s\n' "$USB"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n' "$INITRD"
printf 'scope       : diagnose GRUB linux/initrd/boot handoff\n\n'

( cd "$OVERLAY_DIR" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../phase1-b28-overlay.cpio.gz" )
[ -s "$OVERLAY" ] || fail "failed to build overlay: $OVERLAY"
cat "$INITRD" "$OVERLAY" > "$COMBINED"
[ -s "$COMBINED" ] || fail "failed to build combined initrd: $COMBINED"

printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Initrd SHA256:\n'
sha256sum "$INITRD"
printf 'Overlay SHA256:\n'
sha256sum "$OVERLAY"
printf 'Combined initrd SHA256:\n'
sha256sum "$COMBINED"
printf '\nThis will erase the selected USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART1" ] || fail "partition did not appear: $PART1"

sudo mkfs.vfat -F 32 -n PHASE1B28 "$PART1"
sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1/evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"
sudo cp "$OVERLAY" "$MNT/boot/phase1/phase1-b28-overlay.cpio.gz"
sudo cp "$COMBINED" "$MNT/boot/phase1/phase1-b28-combined-initrd.img"

KERNEL_SIZE="$(stat -c %s "$KERNEL" 2>/dev/null || stat -f %z "$KERNEL")"
INITRD_SIZE="$(stat -c %s "$INITRD" 2>/dev/null || stat -f %z "$INITRD")"
COMBINED_SIZE="$(stat -c %s "$COMBINED" 2>/dev/null || stat -f %z "$COMBINED")"

sudo tee "$MNT/phase1/evidence/b28-prep.env" >/dev/null <<EOF
BASE1_B28_KERNEL=/boot/phase1/vmlinuz
BASE1_B28_INITRD=/boot/phase1/initrd.img
BASE1_B28_OVERLAY=/boot/phase1/phase1-b28-overlay.cpio.gz
BASE1_B28_COMBINED_INITRD=/boot/phase1/phase1-b28-combined-initrd.img
BASE1_B28_KERNEL_SIZE=$KERNEL_SIZE
BASE1_B28_INITRD_SIZE=$INITRD_SIZE
BASE1_B28_COMBINED_INITRD_SIZE=$COMBINED_SIZE
BASE1_B28_EXPECTED_RESULT=phase1_b28_linux_init_seen
EOF

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=-1
set default=0
set pager=1
set color_normal=white/black
set color_highlight=black/light-gray
terminal_input console
terminal_output console

menuentry "B28 File check" {
    clear
    echo "B28 file check"
    echo ""
    ls -lh /boot/phase1
    echo ""
    cat /phase1/evidence/b28-prep.env
    echo ""
    echo "ESC returns."
    sleep --interruptible 999
}

menuentry "B28 Test linux command only" {
    clear
    echo "B28 test: before linux command"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=30
    echo "B28 test: linux command returned"
    echo "If you see this line, kernel load command completed."
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B28 Test linux plus initrd commands" {
    clear
    echo "B28 test: before linux command"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=30
    echo "B28 test: linux command returned"
    echo "B28 test: before initrd command"
    initrd /boot/phase1/phase1-b28-combined-initrd.img
    echo "B28 test: initrd command returned"
    echo "No boot performed in this entry. ESC returns."
    sleep --interruptible 999
}

menuentry "B28 Boot combined initrd" {
    clear
    echo "B28 boot: combined initrd"
    echo "Target: phase1_b28_linux_init_seen"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=30
    echo "linux returned; loading combined initrd"
    initrd /boot/phase1/phase1-b28-combined-initrd.img
    echo "initrd returned; booting now"
    boot
}

menuentry "B28 Boot split initrd overlay" {
    clear
    echo "B28 boot: split initrd + overlay"
    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init nomodeset loglevel=7 panic=30
    echo "linux returned; loading split initrds"
    initrd /boot/phase1/initrd.img /boot/phase1/phase1-b28-overlay.cpio.gz
    echo "initrd returned; booting now"
    boot
}

menuentry "B28 Boot no initrd kernel only" {
    clear
    echo "B28 boot: kernel only, expect panic if handoff works"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset loglevel=7 panic=30
    echo "linux returned; booting kernel without initrd"
    boot
}

menuentry "B28 GRUB fallback" {
    clear
    echo "phase1 6.0.0 ready"
    echo "B28 GRUB fallback console"
    echo "Use if Linux handoff remains blocked."
    sleep --interruptible 999
}

menuentry "Reboot" { reboot }
menuentry "Power off" { halt }
EOF
sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

printf 'Installing GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B28_LINUX_HANDOFF_TARGET=$USB
BASE1_B28_LINUX_HANDOFF_PARTITION=$PART1
BASE1_B28_KERNEL=$KERNEL
BASE1_B28_INITRD=$INITRD
BASE1_B28_OVERLAY=$OVERLAY
BASE1_B28_COMBINED_INITRD=$COMBINED
BASE1_B28_RESULT=prepared
BASE1_B28_EXPECTED_NEXT_RESULT=phase1_b28_linux_init_seen
EOF

printf '\nDONE: B28 Linux handoff diagnostic USB prepared on %s\n' "$USB"
printf 'Boot path: Libreboot -> Load SeaBIOS payload.\n'
printf 'Use B28 Test linux command only first.\n'
