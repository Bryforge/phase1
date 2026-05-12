#!/usr/bin/env bash
set -euo pipefail

QEMU_SHARE="$(brew --prefix qemu)/share/qemu"
OVMF_CODE="$QEMU_SHARE/edk2-x86_64-code.fd"
IMG="build/phase1-uefi.img"

if [ ! -f "$OVMF_CODE" ]; then
  echo "Missing UEFI firmware: $OVMF_CODE"
  exit 1
fi

if [ ! -f "$IMG" ]; then
  echo "Missing image: $IMG"
  echo "Rebuild build/phase1-uefi.img first."
  exit 1
fi

qemu-system-x86_64 \
  -machine q35,accel=tcg \
  -m 4096 \
  -smp 4 \
  -drive if=pflash,format=raw,unit=0,readonly=on,file="$OVMF_CODE" \
  -drive if=none,id=phase1usb,format=raw,file="$IMG" \
  -device qemu-xhci \
  -device usb-storage,drive=phase1usb,bootindex=1 \
  -boot menu=on \
  -vga virtio \
  -display cocoa,zoom-to-fit=on \
  -net none
