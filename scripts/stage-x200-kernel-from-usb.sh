#!/usr/bin/env bash
# Stage the Phase1/X200 kernel from an already prepared USB.
#
# Usage:
#   bash scripts/stage-x200-kernel-from-usb.sh /dev/sdb
#
# Use this when /boot on the host does not contain a readable matching
# vmlinuz/initrd pair, but the current Phase1 USB already has a known-good
# /boot/phase1/vmlinuz artifact.

set -euo pipefail

USB="${1:-}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$REPO_ROOT/build/linux/alpine-netboot"
KERNEL_OUT="$OUT_DIR/vmlinuz"
INITRD_OUT="$OUT_DIR/initrd.img"
SOURCE_ENV="$OUT_DIR/phase1-source.env"

fail() { printf 'stage-x200-kernel-from-usb: %s\n' "$1" >&2; exit 1; }
part1() {
  case "$1" in
    /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;;
    /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;;
    *) fail "use a whole disk like /dev/sdb" ;;
  esac
}

[ -n "$USB" ] || fail "usage: bash scripts/stage-x200-kernel-from-usb.sh /dev/sdb"
[ -b "$USB" ] || fail "not a block device: $USB"
command -v sudo >/dev/null 2>&1 || fail "missing sudo"
command -v mount >/dev/null 2>&1 || fail "missing mount"
command -v umount >/dev/null 2>&1 || fail "missing umount"
command -v sha256sum >/dev/null 2>&1 || fail "missing sha256sum"
command -v findmnt >/dev/null 2>&1 || fail "missing findmnt"

ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in
  "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing root filesystem device: $ROOT_SRC" ;;
esac

PART="$(part1 "$USB")"
[ -b "$PART" ] || fail "partition not found: $PART"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

sudo mount -o ro "$PART" "$MNT"
USB_KERNEL="$MNT/boot/phase1/vmlinuz"
USB_INITRD=""
if [ -f "$MNT/boot/phase1/phase1-b42-native-stable-safe-color-utf8.img" ]; then
  USB_INITRD="$MNT/boot/phase1/phase1-b42-native-stable-safe-color-utf8.img"
elif [ -f "$MNT/boot/phase1/initrd.img" ]; then
  USB_INITRD="$MNT/boot/phase1/initrd.img"
fi

[ -f "$USB_KERNEL" ] || fail "missing USB kernel: /boot/phase1/vmlinuz"

mkdir -p "$OUT_DIR"
cp "$USB_KERNEL" "$KERNEL_OUT"
if [ -n "$USB_INITRD" ]; then
  cp "$USB_INITRD" "$INITRD_OUT"
else
  : > "$INITRD_OUT"
fi

KERNEL_SHA="$(sha256sum "$KERNEL_OUT" | awk '{print $1}')"
INITRD_SHA="$(sha256sum "$INITRD_OUT" | awk '{print $1}')"

cat > "$SOURCE_ENV" <<EOF
BASE1_X200_KERNEL_STAGE_MODE=prepared-usb-copy
BASE1_X200_KERNEL_SOURCE=$USB_KERNEL
BASE1_X200_INITRD_SOURCE=${USB_INITRD:-none}
BASE1_X200_KERNEL_OUT=$KERNEL_OUT
BASE1_X200_INITRD_OUT=$INITRD_OUT
BASE1_X200_KERNEL_SHA256=$KERNEL_SHA
BASE1_X200_INITRD_SHA256=$INITRD_SHA
BASE1_X200_KERNEL_STAGE_CLAIM=known-good-usb-artifact-not-new-hardware-claim
EOF

cleanup
trap - EXIT INT TERM

printf 'DONE: staged kernel from prepared USB.\n'
printf 'kernel: %s\n' "$KERNEL_OUT"
printf 'initrd : %s\n' "$INITRD_OUT"
printf 'source: %s\n' "$SOURCE_ENV"
ls -lh "$OUT_DIR"
file "$KERNEL_OUT" || true
