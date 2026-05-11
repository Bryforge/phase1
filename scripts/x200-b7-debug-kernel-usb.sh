#!/usr/bin/env sh
# Phase1 / Base1 X200 B7 debug USB kernel+initrd writer.
# This creates multiple GRUB entries for old Libreboot/X200 hardware.
# It erases only the selected USB disk and refuses to run without YES_WRITE_USB.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
KERNEL="${BASE1_B7_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B7_INITRD:-build/linux/alpine-netboot/initrd.img}"
OUT_DIR="${BASE1_B7_DEBUG_OUT:-build/base1-b7-debug-kernel-usb}"

fail() { printf 'x200-b7-debug-kernel-usb: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

case "$USB" in
  /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) PART="${USB}1" ;;
  /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) PART="${USB}p1" ;;
  *) fail "use whole disk path only, for example /dev/sdb" ;;
esac

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "run: sh scripts/x200-b7-debug-kernel-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
[ "$USB" != "/dev/sda" ] || fail "refusing /dev/sda because it is commonly the internal disk"
[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing initrd: $INITRD"

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum; do need_cmd "$cmd"; done

MNT="$(mktemp -d)"
REPORT="$OUT_DIR/b7-debug-kernel-usb.env"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

printf 'PHASE1 B7 DEBUG USB WRITER\n\n'
printf 'target disk : %s\n' "$USB"
printf 'partition   : %s\n' "$PART"
printf 'kernel      : %s\n' "$KERNEL"
printf 'initrd      : %s\n\n' "$INITRD"
printf 'This will erase the USB target.\n\n'

sudo umount "${USB}"* 2>/dev/null || true
sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2
[ -b "$PART" ] || fail "partition did not appear: $PART"

sudo mkfs.vfat -F 32 -n PHASE1B7D "$PART"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=15
set default=0
terminal_output console
set gfxpayload=text

menuentry "Phase1 B7 debug kernel/initrd - text nomodeset" {
    terminal_output console
    set gfxpayload=text
    echo "Phase1 B7 debug: text mode + nomodeset"
    echo "If kernel lines appear after this, record: boot_started"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 earlyprintk=vga keep_bootcon
    initrd /boot/phase1/initrd.img
}

menuentry "Phase1 B7 debug kernel/initrd - noapic acpi off" {
    terminal_output console
    set gfxpayload=text
    echo "Phase1 B7 debug: noapic nolapic acpi=off"
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal noapic nolapic acpi=off loglevel=7 earlyprintk=vga keep_bootcon
    initrd /boot/phase1/initrd.img
}

menuentry "Phase1 B7 debug kernel only - expected panic is useful" {
    terminal_output console
    set gfxpayload=text
    echo "Phase1 B7 debug: kernel only, no initrd"
    echo "If kernel panic appears, kernel execution is proven."
    linux /boot/phase1/vmlinuz console=tty0 nomodeset vga=normal loglevel=7 earlyprintk=vga keep_bootcon
}

menuentry "Phase1 B6 marker fallback" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B7 fallback marker reached from external USB"
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"
sudo tee "$MNT/PHASE1-B7-DEBUG.txt" >/dev/null <<'EOF'
Phase1 B7 debug USB for X200 Libreboot/Coreboot path.
Evidence states:
- boot_started: kernel output or kernel panic appears
- blocked: GRUB loads initrd but never transfers visibly to kernel
- phase1_marker_seen: marker fallback appears
No installer claim. No internal disk claim. No daily-driver claim.
EOF

sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$USB"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

mkdir -p "$OUT_DIR"
cat > "$REPORT" <<EOF
BASE1_B7_DEBUG_USB_TARGET=$USB
BASE1_B7_DEBUG_USB_PARTITION=$PART
BASE1_B7_DEBUG_USB_KERNEL=$KERNEL
BASE1_B7_DEBUG_USB_INITRD=$INITRD
BASE1_B7_DEBUG_USB_RESULT=prepared
BASE1_B7_DEBUG_USB_EXPECTED_NEXT_RESULT=boot_started_or_blocked
BASE1_B7_DEBUG_USB_CLAIM=not_claimed
EOF

printf '\nDONE: B7 debug USB prepared on %s\n' "$USB"
printf 'Try entries in order. If stuck after initrd again, try entry 2, then entry 3.\n'
