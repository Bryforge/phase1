#!/usr/bin/env bash
# Stage the current host kernel/initrd for Phase1 X200 boot prep.
#
# Usage:
#   bash scripts/stage-x200-kernel.sh
#
# This script copies the current host kernel and initrd from /boot into the
# build/linux/alpine-netboot artifact path used by Phase1 X200 USB writers.
# It may ask for sudo because /boot kernel artifacts can be protected.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

OUT_DIR="build/linux/alpine-netboot"
KERNEL_SRC="/boot/vmlinuz-$(uname -r)"
INITRD_SRC="/boot/initrd.img-$(uname -r)"
KERNEL_OUT="$OUT_DIR/vmlinuz"
INITRD_OUT="$OUT_DIR/initrd.img"
SOURCE_ENV="$OUT_DIR/phase1-source.env"

printf 'Phase1 X200 kernel staging\n'
printf 'repo          : %s\n' "$REPO_ROOT"
printf 'kernel source : %s\n' "$KERNEL_SRC"
printf 'initrd source : %s\n' "$INITRD_SRC"
printf 'output dir    : %s\n\n' "$OUT_DIR"

if [ ! -f "$KERNEL_SRC" ]; then
  printf 'ERROR: missing kernel source: %s\n' "$KERNEL_SRC" >&2
  exit 1
fi

if [ ! -f "$INITRD_SRC" ]; then
  printf 'ERROR: missing initrd source: %s\n' "$INITRD_SRC" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

sudo cp "$KERNEL_SRC" "$KERNEL_OUT"
sudo cp "$INITRD_SRC" "$INITRD_OUT"
sudo chown -R "$(id -u):$(id -g)" "$OUT_DIR"

KERNEL_SHA="$(sha256sum "$KERNEL_OUT" | awk '{print $1}')"
INITRD_SHA="$(sha256sum "$INITRD_OUT" | awk '{print $1}')"

cat > "$SOURCE_ENV" <<EOF
BASE1_X200_KERNEL_STAGE_MODE=host-current-kernel
BASE1_X200_KERNEL_SOURCE=$KERNEL_SRC
BASE1_X200_INITRD_SOURCE=$INITRD_SRC
BASE1_X200_KERNEL_OUT=$KERNEL_OUT
BASE1_X200_INITRD_OUT=$INITRD_OUT
BASE1_X200_KERNEL_SHA256=$KERNEL_SHA
BASE1_X200_INITRD_SHA256=$INITRD_SHA
BASE1_X200_KERNEL_STAGE_CLAIM=not_claimed
EOF

printf '\n--- staged artifacts ---\n'
ls -lh "$OUT_DIR"

printf '\n--- kernel file type ---\n'
file "$KERNEL_OUT"

printf '\n--- hashes ---\n'
sha256sum "$KERNEL_OUT" "$INITRD_OUT"

printf '\nDONE: kernel/initrd staged for Phase1 X200 boot prep.\n'
printf 'source env: %s\n' "$SOURCE_ENV"
