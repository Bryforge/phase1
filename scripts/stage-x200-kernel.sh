#!/usr/bin/env bash
# Stage a host kernel/initrd for Phase1 X200 boot prep.
#
# Usage:
#   bash scripts/stage-x200-kernel.sh
#   BASE1_X200_KERNEL_SOURCE=/path/vmlinuz BASE1_X200_INITRD_SOURCE=/path/initrd.img bash scripts/stage-x200-kernel.sh
#
# The script first tries explicit env paths, then the running kernel from
# uname -r, then the newest matching /boot/vmlinuz-* + /boot/initrd.img-* pair.
# It may ask for sudo because /boot kernel artifacts can be protected.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

OUT_DIR="${BASE1_X200_KERNEL_OUT_DIR:-build/linux/alpine-netboot}"
KERNEL_OUT="$OUT_DIR/vmlinuz"
INITRD_OUT="$OUT_DIR/initrd.img"
SOURCE_ENV="$OUT_DIR/phase1-source.env"

fail() { printf 'stage-x200-kernel: %s\n' "$1" >&2; exit 1; }

choose_pair() {
  if [ -n "${BASE1_X200_KERNEL_SOURCE:-}" ] && [ -n "${BASE1_X200_INITRD_SOURCE:-}" ]; then
    printf '%s\n%s\nmanual-env\n' "$BASE1_X200_KERNEL_SOURCE" "$BASE1_X200_INITRD_SOURCE"
    return 0
  fi

  local rel kernel initrd candidate suffix
  rel="$(uname -r)"
  kernel="/boot/vmlinuz-$rel"
  initrd="/boot/initrd.img-$rel"
  if [ -f "$kernel" ] && [ -f "$initrd" ]; then
    printf '%s\n%s\nhost-running-kernel\n' "$kernel" "$initrd"
    return 0
  fi

  while IFS= read -r candidate; do
    [ -n "$candidate" ] || continue
    suffix="${candidate#/boot/vmlinuz-}"
    initrd="/boot/initrd.img-$suffix"
    if [ -f "$candidate" ] && [ -f "$initrd" ]; then
      printf '%s\n%s\nnewest-boot-pair\n' "$candidate" "$initrd"
      return 0
    fi
  done <<EOF
$(ls -1 /boot/vmlinuz-* 2>/dev/null | sort -Vr || true)
EOF

  return 1
}

printf 'Phase1 X200 kernel staging\n'
printf 'repo       : %s\n' "$REPO_ROOT"
printf 'output dir : %s\n\n' "$OUT_DIR"

PAIR="$(choose_pair || true)"
if [ -z "$PAIR" ]; then
  printf 'No matching kernel/initrd pair found. /boot contents:\n' >&2
  ls -lh /boot 2>/dev/null >&2 || true
  fail 'set BASE1_X200_KERNEL_SOURCE and BASE1_X200_INITRD_SOURCE manually'
fi

KERNEL_SRC="$(printf '%s\n' "$PAIR" | sed -n '1p')"
INITRD_SRC="$(printf '%s\n' "$PAIR" | sed -n '2p')"
SOURCE_MODE="$(printf '%s\n' "$PAIR" | sed -n '3p')"

printf 'source mode   : %s\n' "$SOURCE_MODE"
printf 'kernel source : %s\n' "$KERNEL_SRC"
printf 'initrd source : %s\n' "$INITRD_SRC"

if [ ! -f "$KERNEL_SRC" ]; then
  fail "missing kernel source: $KERNEL_SRC"
fi
if [ ! -f "$INITRD_SRC" ]; then
  fail "missing initrd source: $INITRD_SRC"
fi

mkdir -p "$OUT_DIR"
sudo cp "$KERNEL_SRC" "$KERNEL_OUT"
sudo cp "$INITRD_SRC" "$INITRD_OUT"
sudo chown -R "$(id -u):$(id -g)" "$OUT_DIR"

KERNEL_SHA="$(sha256sum "$KERNEL_OUT" | awk '{print $1}')"
INITRD_SHA="$(sha256sum "$INITRD_OUT" | awk '{print $1}')"

cat > "$SOURCE_ENV" <<EOF
BASE1_X200_KERNEL_STAGE_MODE=$SOURCE_MODE
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
