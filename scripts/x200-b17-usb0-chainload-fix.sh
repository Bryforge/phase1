#!/usr/bin/env sh
# Phase1 / Base1 X200 B17 usb0 chainload fix.
#
# Purpose:
#   Patch an already-prepared B17 USB so GRUB chainloads the raw bootsector
#   using the device name actually observed on Libreboot: (usb0,msdos2).
#
# Safety:
#   This does not repartition, format, or rewrite the bootsector. It mounts
#   partition 1 and replaces only the GRUB configuration. It still requires
#   YES_WRITE_USB because it writes to the selected USB filesystem.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"

fail() { printf 'x200-b17-usb0-chainload-fix: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

partition_one() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

partition_two() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp2\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s2\n' "$1" ;;
    *) fail "use a whole-disk path such as /dev/sdb, not a partition" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "usage: sh scripts/x200-b17-usb0-chainload-fix.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"

for cmd in sudo mount umount mkdir tee dd tail od sync; do
  need_cmd "$cmd"
done

PART1="$(partition_one "$USB")"
PART2="$(partition_two "$USB")"
[ -b "$PART1" ] || fail "missing GRUB partition: $PART1"
[ -b "$PART2" ] || fail "missing raw bootsector partition: $PART2"

SIG="$(sudo dd if="$PART2" bs=512 count=1 status=none | tail -c 2 | od -An -tx1 | tr -d ' \n')"
[ "$SIG" = "55aa" ] || fail "partition 2 does not contain a bootsector signature; got $SIG"

MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

sudo mount "$PART1" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=30
set default=0
set pager=1

menuentry "B17-USB0 Phase1 raw bootsector chainload - observed Libreboot device" {
    echo "Phase1 Base1 B17: chainloading observed Libreboot USB device"
    echo "Target evidence: phase1_bootsector_seen"
    echo "Using: set root=(usb0,msdos2); chainloader +1"
    set root=(usb0,msdos2)
    chainloader +1
    boot
}

menuentry "B17-USB0 direct chainloader syntax" {
    echo "Phase1 Base1 B17: direct usb0/msdos2 chainloader syntax"
    echo "Target evidence: phase1_bootsector_seen"
    chainloader (usb0,msdos2)+1
    boot
}

menuentry "B17-HD0 fallback chainload" {
    echo "Phase1 Base1 B17: hd0/msdos2 fallback"
    chainloader (hd0,msdos2)+1
    boot
}

menuentry "B17-HD1 fallback chainload" {
    echo "Phase1 Base1 B17: hd1/msdos2 fallback"
    chainloader (hd1,msdos2)+1
    boot
}

menuentry "B17 device listing" {
    clear
    echo "Phase1 Base1 B17 device listing"
    echo "Known good observed target from prior run: (usb0,msdos2)"
    echo "Visible GRUB devices:"
    ls
    echo ""
    echo "Press ESC to return."
    sleep --interruptible 999
}

menuentry "B11 fallback - Phase1 GRUB-native operations console" {
    clear
    echo "phase1 6.0.0 ready"
    echo "Base1 external USB GRUB-native console"
    echo "B11 candidate result: phase1_grub_console_seen"
    echo "This fallback proves the external USB/GRUB path remains active."
    echo "It is not a kernel boot claim."
    sleep --interruptible 999
}

menuentry "B6 marker fallback - known good external USB proof" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B17 fallback marker reached from external USB"
    echo "This is not an installer, hardening, or daily-driver claim."
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

printf 'B17 usb0 chainload fix installed on %s\n' "$USB"
printf 'Verified raw bootsector signature on %s: %s\n' "$PART2" "$SIG"
printf 'Boot and choose: B17-USB0 Phase1 raw bootsector chainload - observed Libreboot device\n'
