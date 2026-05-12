#!/usr/bin/env sh
# Phase1 / Base1 X200 B23 host GNU/Linux artifact staging.
#
# Purpose:
#   Prepare local kernel/initrd artifacts for the B23 GNU/Linux runtime USB.
#   This avoids another back-and-forth when build/linux artifacts are absent
#   from a fresh clone because build artifacts are normally ignored.
#
# Source preference:
#   1. BASE1_B23_KERNEL_SOURCE and BASE1_B23_INITRD_SOURCE when set.
#   2. /boot/vmlinuz-$(uname -r) and /boot/initrd.img-$(uname -r).
#   3. The newest /boot/vmlinuz-* and matching /boot/initrd.img-*.
#
# Output:
#   build/linux/alpine-netboot/vmlinuz
#   build/linux/alpine-netboot/initrd.img
#   build/linux/alpine-netboot/phase1-source.env
#
# This script only copies local files into build/. It does not write disks.

set -eu

OUT_DIR="${BASE1_B23_ARTIFACT_DIR:-build/linux/alpine-netboot}"
KERNEL_OUT="$OUT_DIR/vmlinuz"
INITRD_OUT="$OUT_DIR/initrd.img"
SOURCE_ENV="$OUT_DIR/phase1-source.env"

fail() { printf 'x200-b23-stage-host-gnulinux: %s\n' "$1" >&2; exit 1; }
need_cmd() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }

need_cmd uname
need_cmd mkdir
need_cmd cp
need_cmd ls
need_cmd sha256sum
need_cmd date

KERNEL_SOURCE="${BASE1_B23_KERNEL_SOURCE:-}"
INITRD_SOURCE="${BASE1_B23_INITRD_SOURCE:-}"

if [ -z "$KERNEL_SOURCE" ] || [ -z "$INITRD_SOURCE" ]; then
  REL="$(uname -r)"
  if [ -f "/boot/vmlinuz-$REL" ] && [ -f "/boot/initrd.img-$REL" ]; then
    KERNEL_SOURCE="/boot/vmlinuz-$REL"
    INITRD_SOURCE="/boot/initrd.img-$REL"
  fi
fi

if [ -z "$KERNEL_SOURCE" ] || [ -z "$INITRD_SOURCE" ]; then
  CANDIDATE="$(ls -1 /boot/vmlinuz-* 2>/dev/null | sort | tail -n 1 || true)"
  [ -n "$CANDIDATE" ] || fail "no /boot/vmlinuz-* found; set BASE1_B23_KERNEL_SOURCE and BASE1_B23_INITRD_SOURCE"
  SUFFIX="${CANDIDATE#/boot/vmlinuz-}"
  MATCHING_INITRD="/boot/initrd.img-$SUFFIX"
  [ -f "$MATCHING_INITRD" ] || fail "found $CANDIDATE but missing $MATCHING_INITRD"
  KERNEL_SOURCE="$CANDIDATE"
  INITRD_SOURCE="$MATCHING_INITRD"
fi

[ -f "$KERNEL_SOURCE" ] || fail "kernel source not found: $KERNEL_SOURCE"
[ -f "$INITRD_SOURCE" ] || fail "initrd source not found: $INITRD_SOURCE"

mkdir -p "$OUT_DIR"
cp "$KERNEL_SOURCE" "$KERNEL_OUT"
cp "$INITRD_SOURCE" "$INITRD_OUT"

KERNEL_SHA="$(sha256sum "$KERNEL_OUT" | awk '{print $1}')"
INITRD_SHA="$(sha256sum "$INITRD_OUT" | awk '{print $1}')"

cat > "$SOURCE_ENV" <<EOF
BASE1_B23_GNULINUX_SOURCE_MODE=host-local-copy
BASE1_B23_GNULINUX_KERNEL_SOURCE=$KERNEL_SOURCE
BASE1_B23_GNULINUX_INITRD_SOURCE=$INITRD_SOURCE
BASE1_B23_GNULINUX_KERNEL_OUT=$KERNEL_OUT
BASE1_B23_GNULINUX_INITRD_OUT=$INITRD_OUT
BASE1_B23_GNULINUX_KERNEL_SHA256=$KERNEL_SHA
BASE1_B23_GNULINUX_INITRD_SHA256=$INITRD_SHA
BASE1_B23_GNULINUX_STAGED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
BASE1_B23_GNULINUX_CLAIM=not_claimed
EOF

printf 'B23 GNU/Linux artifacts staged.\n'
printf 'kernel: %s -> %s\n' "$KERNEL_SOURCE" "$KERNEL_OUT"
printf 'initrd: %s -> %s\n' "$INITRD_SOURCE" "$INITRD_OUT"
printf 'source: %s\n' "$SOURCE_ENV"
sha256sum "$KERNEL_OUT" "$INITRD_OUT"
