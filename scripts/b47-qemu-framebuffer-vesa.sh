#!/usr/bin/env sh
# Phase1 B47 QEMU VESA framebuffer runner.
#
# Usage:
#   sh scripts/b47-qemu-framebuffer-vesa.sh
#
# The first B47 run reached BusyBox but /dev/fb0 was not active. This runner
# tries legacy VESA mode setup with QEMU std VGA.

set -eu

OUT_DIR="${BASE1_B47_OUT:-build/base1-b47-qemu-framebuffer-lab}"
KERNEL="${BASE1_B47_KERNEL:-build/linux/alpine-netboot/vmlinuz}"
INITRD="$OUT_DIR/phase1-b47-framebuffer-lab.img"
DISPLAY_BACKEND="${BASE1_B47_DISPLAY:-gtk}"
APPEND="console=tty0 rdinit=/init vga=791 video=vesafb:mtrr:3,ywrap clocksource=hpet tsc=unstable"

fail() { printf 'b47-qemu-framebuffer-vesa: %s\n' "$1" >&2; exit 1; }
command -v qemu-system-x86_64 >/dev/null 2>&1 || fail "missing qemu-system-x86_64"
[ -f "$KERNEL" ] || fail "missing kernel: $KERNEL"
[ -f "$INITRD" ] || fail "missing initrd: $INITRD; run scripts/b47-qemu-framebuffer-lab.sh build first"

printf 'B47 QEMU VESA framebuffer attempt\n'
printf 'kernel : %s\n' "$KERNEL"
printf 'initrd : %s\n' "$INITRD"
printf 'append : %s\n' "$APPEND"
printf '\nIf blit fails inside QEMU shell, run:\n'
printf '  cat /proc/fb\n'
printf '  ls -l /dev/fb0\n'
printf '  dmesg | grep -i fb\n'
printf '\nLaunching QEMU...\n'

qemu-system-x86_64 \
  -m 512M \
  -vga std \
  -kernel "$KERNEL" \
  -initrd "$INITRD" \
  -append "$APPEND" \
  -display "$DISPLAY_BACKEND" \
  -no-reboot
