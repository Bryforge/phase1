#!/usr/bin/env sh
# Phase1 / Base1 X200 B23 GNU/Linux runtime preflight.
#
# Purpose:
#   Check that the X200 has the local tools and kernel/initrd artifacts needed
#   before preparing the B23 GNU/Linux runtime USB. This avoids another reboot
#   loop caused by missing host-side files.

set -eu

KERNEL="${BASE1_B23_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="${BASE1_B23_INITRD:-build/linux/alpine-netboot/initrd.img}"
OUT_DIR="${BASE1_B23_PREFLIGHT_OUT:-build/base1-b23-gnulinux-runtime}"
REPORT="$OUT_DIR/b23-preflight.env"
MISSING=""

mkdir -p "$OUT_DIR"

printf 'PHASE1 B23 GNU/LINUX PREFLIGHT\n\n'
printf 'kernel: %s\n' "$KERNEL"
printf 'initrd: %s\n\n' "$INITRD"

for cmd in sh git sudo parted mkfs.vfat mount umount sync mkdir cp tee grub-install sha256sum find cpio gzip chmod date; do
  if command -v "$cmd" >/dev/null 2>&1; then
    printf 'ok: %s\n' "$cmd"
  else
    printf 'missing: %s\n' "$cmd"
    MISSING="$MISSING $cmd"
  fi
done

KERNEL_PRESENT=no
INITRD_PRESENT=no
[ -f "$KERNEL" ] && KERNEL_PRESENT=yes
[ -f "$INITRD" ] && INITRD_PRESENT=yes

printf '\nartifact check:\n'
printf 'kernel_present=%s\n' "$KERNEL_PRESENT"
printf 'initrd_present=%s\n' "$INITRD_PRESENT"

if [ "$KERNEL_PRESENT" = yes ]; then
  sha256sum "$KERNEL"
fi
if [ "$INITRD_PRESENT" = yes ]; then
  sha256sum "$INITRD"
fi

cat > "$REPORT" <<EOF
BASE1_B23_PREFLIGHT_KERNEL=$KERNEL
BASE1_B23_PREFLIGHT_INITRD=$INITRD
BASE1_B23_PREFLIGHT_KERNEL_PRESENT=$KERNEL_PRESENT
BASE1_B23_PREFLIGHT_INITRD_PRESENT=$INITRD_PRESENT
BASE1_B23_PREFLIGHT_MISSING_COMMANDS=$MISSING
EOF

printf '\nreport: %s\n' "$REPORT"

if [ -n "$MISSING" ]; then
  printf '\nInstall missing commands before preparing USB.\n'
  printf 'On Trisquel/Debian style systems, likely packages include:\n'
  printf '  sudo apt update\n'
  printf '  sudo apt install -y git coreutils util-linux parted dosfstools grub-pc-bin cpio gzip\n'
  exit 1
fi

if [ "$KERNEL_PRESENT" != yes ] || [ "$INITRD_PRESENT" != yes ]; then
  printf '\nKernel/initrd artifacts are missing. Try staging from the host first:\n'
  printf '  sh scripts/x200-b23-stage-host-gnulinux.sh\n'
  exit 1
fi

printf '\npreflight: pass\n'
printf 'next: sh scripts/x200-b23-gnulinux-runtime-usb.sh /dev/sdb YES_WRITE_USB\n'
