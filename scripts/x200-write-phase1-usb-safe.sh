#!/usr/bin/env sh
set -eu

USB="${1:-}"
IMG="${PHASE1_USB_IMAGE:-build/phase1-uefi.img}"

fail() {
  printf 'x200-usb-write: %s\n' "$1" >&2
  exit 1
}

[ -n "$USB" ] || fail "usage: sh scripts/x200-write-phase1-usb-safe.sh /dev/sdX"
case "$USB" in
  /dev/sd?) ;;
  *) fail "USB must be a whole disk like /dev/sdb, not /dev/sdb1" ;;
esac

BASE=$(basename "$USB")

[ -f "$IMG" ] || fail "missing image: $IMG"
[ -b "$USB" ] || fail "not a block device: $USB"

printf 'PHASE1 X200 SAFE USB WRITER\n\n'
printf 'image : %s\n' "$IMG"
printf 'target: %s\n\n' "$USB"

printf 'Image size/checksum:\n'
ls -lh "$IMG"
sha256sum "$IMG" || true

printf '\nTarget device info:\n'
if [ -r "/sys/block/$BASE/device/model" ]; then
  printf 'model    : '
  cat "/sys/block/$BASE/device/model"
fi
if [ -r "/sys/block/$BASE/removable" ]; then
  printf 'removable: '
  cat "/sys/block/$BASE/removable"
fi
if [ -r "/sys/block/$BASE/size" ]; then
  sectors=$(cat "/sys/block/$BASE/size")
  mb=$((sectors * 512 / 1024 / 1024))
  printf 'size_mb  : %s\n' "$mb"
fi

printf '\nKernel partition list for %s:\n' "$BASE"
awk -v b="$BASE" '$4 == b || index($4, b) == 1 {print}' /proc/partitions || true

printf '\nMounted partitions on target:\n'
grep "^/dev/$BASE" /proc/mounts || printf 'none\n'

ROOTSRC=$(awk '$2 == "/" {print $1}' /proc/mounts | head -n 1)
printf '\nRoot filesystem source: %s\n' "$ROOTSRC"
case "$ROOTSRC" in
  "$USB"|"$USB"*) fail "target appears to contain the running root filesystem" ;;
esac

printf '\nThis will OVERWRITE the whole USB disk: %s\n' "$USB"
printf 'Type exactly WRITE_%s to continue: ' "$BASE"
read ans

[ "$ans" = "WRITE_$BASE" ] || fail "cancelled"

printf '\nUnmounting target partitions if mounted...\n'
awk -v b="/dev/$BASE" '$1 ~ ("^" b "[0-9]+") {print $2}' /proc/mounts | while read mnt; do
  [ -n "$mnt" ] && sudo umount "$mnt" || true
done

printf '\nWriting image. Do not remove USB.\n'
sudo dd if="$IMG" of="$USB" bs=4M status=progress conv=fsync
sync

printf '\nUSB write complete: %s\n' "$USB"
printf 'Now reboot, press F12 on the X200, and choose the USB device.\n'
