#!/usr/bin/env sh
# X200 Libreboot GRUB marker USB helper.
#
# This intentionally creates a tiny Libreboot/BIOS-readable GRUB USB that
# displays the Phase1 marker. It does not install Base1, modify host boot
# settings, claim OS boot, claim hardware validation, or touch internal disks
# unless the operator passes the wrong target disk.

set -eu

DISK=${1:-}
CONFIRM=${2:-}
LABEL=PHASE1X200
MNT=${MNT:-/tmp/phase1-x200-usb}

fail() {
  printf 'x200-libreboot-grub-marker-usb: %s\n' "$1" >&2
  exit 1
}

need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    fail "missing command: $1. On Trisquel try: sudo apt install -y parted dosfstools grub-pc-bin"
  fi
}

[ -n "$DISK" ] || fail 'usage: sh scripts/x200-libreboot-grub-marker-usb.sh /dev/sdX YES_WRITE_USB'
[ "$CONFIRM" = YES_WRITE_USB ] || fail 'refusing without confirmation token: YES_WRITE_USB'

case "$DISK" in
  /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) : ;;
  /dev/mmcblk[0-9]|/dev/nvme[0-9]n[0-9]) : ;;
  *) fail "target must be a whole disk, not a partition: $DISK" ;;
esac

need sudo
need parted
need mkfs.vfat
need grub-install
need mkdir
need tee
need sync

case "$DISK" in
  /dev/mmcblk*|/dev/nvme*) PART="${DISK}p1" ;;
  *) PART="${DISK}1" ;;
esac

printf 'PHASE1 X200 LIBREBOOT GRUB MARKER USB\n\n'
printf 'TARGET DISK : %s\n' "$DISK"
printf 'TARGET PART : %s\n' "$PART"
printf 'THIS WILL ERASE THE USB TARGET ABOVE.\n\n'

sudo umount "${PART}" 2>/dev/null || true
sudo umount "${DISK}" 2>/dev/null || true

sudo parted -s "$DISK" mklabel msdos
sudo parted -s "$DISK" mkpart primary fat32 1MiB 100%
sudo parted -s "$DISK" set 1 boot on

# Give the kernel a moment to notice the new partition table.
sleep 2

sudo mkfs.vfat -F 32 -n "$LABEL" "$PART"

sudo rm -rf "$MNT"
sudo mkdir -p "$MNT"
sudo mount "$PART" "$MNT"
sudo mkdir -p "$MNT/boot/grub"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF_GRUB'
set timeout=30
set default=0

menuentry 'Phase1 Base1 X200 hardware marker' {
    clear
    echo 'phase1 6.0.0 ready'
    echo 'B6 candidate result: phase1_marker_seen'
    echo 'This proves Libreboot/GRUB reached external USB media.'
    echo 'This is not yet a Base1 kernel boot, installer, hardening, or daily-driver claim.'
    echo 'Press Esc or wait to return.'
    sleep --interruptible 60
}

menuentry 'Phase1 Base1 X200 boot menu seen marker' {
    clear
    echo 'phase1 boot menu seen'
    echo 'B6 candidate result: boot_menu_seen'
    sleep --interruptible 30
}
EOF_GRUB

sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$DISK"
sudo sync
sudo umount "$MNT"

printf '\nUSB marker media written.\n'
printf 'Next: reboot X200, choose Libreboot external GRUB search [s].\n'
printf 'If that fails, choose SeaBIOS payload [b] and boot the USB disk.\n'
printf 'When you see "phase1 6.0.0 ready", record B6 as phase1_marker_seen.\n'
