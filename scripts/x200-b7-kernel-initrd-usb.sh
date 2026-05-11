#!/usr/bin/env sh
# Phase1 / Base1 X200 B7 external USB kernel+initrd writer.
#
# DANGER: this intentionally erases the selected USB disk only.
# It refuses to run without YES_WRITE_USB and refuses /dev/sda unless
# PHASE1_ALLOW_SDA=1 is explicitly set.
#
# Scope: external USB boot evidence only.
# Non-claims: no installer claim, no internal disk install claim, no
# recovery-complete claim, no hardening claim, no hardware-readiness claim,
# no daily-driver claim.

set -eu

USB="${1:-/dev/sdb}"
CONFIRM="${2:-}"
PROFILE="${BASE1_B7_PROFILE:-x200-supervisor-lite}"
OUT_DIR="${BASE1_B7_OUT:-build/base1-b7-kernel-usb}"
KERNEL="${BASE1_B7_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B7_INITRD:-build/linux/alpine-netboot/initrd.img}"

fail() {
  printf 'x200-b7-kernel-usb: %s\n' "$1" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "missing command: $1. On Trisquel run: sudo apt update && sudo apt install -y parted dosfstools grub-pc-bin coreutils util-linux"
}

is_whole_disk_path() {
  case "$1" in
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) return 0 ;;
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) return 0 ;;
    *) return 1 ;;
  esac
}

partition_one() {
  case "$1" in
    /dev/nvme*|/dev/mmcblk*) printf '%sp1\n' "$1" ;;
    *) printf '%s1\n' "$1" ;;
  esac
}

[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "refusing to write. Run: sh scripts/x200-b7-kernel-initrd-usb.sh /dev/sdb YES_WRITE_USB"
[ -b "$USB" ] || fail "not a block device: $USB"
is_whole_disk_path "$USB" || fail "use the whole disk, not a partition. Example: /dev/sdb, not /dev/sdb1"

if [ "$USB" = "/dev/sda" ] && [ "${PHASE1_ALLOW_SDA:-0}" != "1" ]; then
  fail "refusing /dev/sda because it is commonly the internal disk. Use /dev/sdb for the USB, or set PHASE1_ALLOW_SDA=1 only if you are absolutely certain."
fi

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing to write root filesystem device: root=$ROOT_SRC target=$USB" ;;
esac

[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing initrd: $INITRD"

for cmd in sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum; do
  need_cmd "$cmd"
done

PART="$(partition_one "$USB")"
MNT="$(mktemp -d)"
REPORT="$OUT_DIR/b7-kernel-usb.env"

cleanup() {
  sudo umount "$MNT" 2>/dev/null || true
  rmdir "$MNT" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

printf 'PHASE1 B7 EXTERNAL USB KERNEL/INITRD WRITER\n\n'
printf 'profile       : %s\n' "$PROFILE"
printf 'target disk   : %s\n' "$USB"
printf 'partition     : %s\n' "$PART"
printf 'kernel        : %s\n' "$KERNEL"
printf 'initrd        : %s\n' "$INITRD"
printf 'WARNING       : this erases the selected USB disk\n\n'

printf 'Kernel SHA256:\n'
sha256sum "$KERNEL"
printf 'Initrd SHA256:\n'
sha256sum "$INITRD"
printf '\nWriting USB now...\n'

sudo umount "${USB}"* 2>/dev/null || true

sudo parted -s "$USB" mklabel msdos
sudo parted -s "$USB" mkpart primary fat32 1MiB 100%
sudo parted -s "$USB" set 1 boot on
sudo partprobe "$USB" 2>/dev/null || true
sync
sleep 2

[ -b "$PART" ] || fail "partition did not appear: $PART"

sudo mkfs.vfat -F 32 -n PHASE1B7 "$PART"
sudo mount "$PART" "$MNT"

sudo mkdir -p "$MNT/boot/grub" "$MNT/grub" "$MNT/boot/phase1" "$MNT/phase1-evidence"
sudo cp "$KERNEL" "$MNT/boot/phase1/vmlinuz"
sudo cp "$INITRD" "$MNT/boot/phase1/initrd.img"

sudo tee "$MNT/boot/grub/grub.cfg" >/dev/null <<'EOF'
set timeout=10
set default=0

menuentry "Phase1 B7 kernel/initrd boot attempt" {
    echo "Phase1 B7: loading kernel/initrd from external USB"
    echo "If the kernel starts, record: boot_started"
    echo "If the marker appears later, record: phase1_marker_seen"
    linux /boot/phase1/vmlinuz console=tty0
    initrd /boot/phase1/initrd.img
}

menuentry "Phase1 B6 marker fallback" {
    echo "phase1 6.0.0 ready"
    echo "B6 candidate result: phase1_marker_seen"
    echo "B7 fallback marker reached from external USB"
    sleep --interruptible 999
}
EOF

sudo cp "$MNT/boot/grub/grub.cfg" "$MNT/grub/grub.cfg"

sudo tee "$MNT/PHASE1-B7-USB.txt" >/dev/null <<EOF
Phase1 B7 external USB kernel/initrd boot attempt.
Profile: $PROFILE
Kernel: /boot/phase1/vmlinuz
Initrd: /boot/phase1/initrd.img
Evidence states: boot_menu_seen, boot_started, phase1_marker_seen, blocked, failed.
No installer claim.
No internal disk install claim.
No recovery-complete claim.
No daily-driver claim.
EOF

printf '\nInstalling GRUB BIOS bootloader to %s...\n' "$USB"
sudo grub-install --target=i386-pc --boot-directory="$MNT/boot" --recheck "$USB"

sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

mkdir -p "$OUT_DIR"
cat > "$REPORT" <<EOF
BASE1_B7_KERNEL_USB_PROFILE=$PROFILE
BASE1_B7_KERNEL_USB_TARGET=$USB
BASE1_B7_KERNEL_USB_PARTITION=$PART
BASE1_B7_KERNEL_USB_KERNEL=$KERNEL
BASE1_B7_KERNEL_USB_INITRD=$INITRD
BASE1_B7_KERNEL_USB_RESULT=prepared
BASE1_B7_KERNEL_USB_EXPECTED_NEXT_RESULT=boot_started
BASE1_B7_KERNEL_USB_CLAIM=not_claimed
BASE1_B7_NON_CLAIM_INSTALLER=1
BASE1_B7_NON_CLAIM_INTERNAL_DISK_INSTALL=1
BASE1_B7_NON_CLAIM_RECOVERY_COMPLETE=1
BASE1_B7_NON_CLAIM_DAILY_DRIVER=1
EOF

printf '\nDONE: B7 USB prepared on %s\n' "$USB"
printf 'Local report: %s\n' "$REPORT"
printf '\nNext: reboot the X200, choose USB/SeaBIOS/GRUB, then select:\n'
printf '  Phase1 B7 kernel/initrd boot attempt\n'
printf '\nEvidence rule:\n'
printf '  If you see kernel/initrd text after selecting it, record B6/B7 result as boot_started.\n'
printf '  If it only returns to marker fallback, record phase1_marker_seen only.\n'
