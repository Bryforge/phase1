#!/usr/bin/env sh
# Verbose runner for B44 Unicode/font augmentation.
#
# The underlying augmentation can take a while because it copies fonts and
# repacks the initramfs. This wrapper keeps the terminal alive with progress
# output and points to the log if anything fails.

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
OUT_DIR="${BASE1_B44_OUT:-build/base1-b44-unicode-font-augment}"
LOG="$OUT_DIR/b44-unicode-font-augment.verbose.log"
PID_FILE="$OUT_DIR/b44-augment.pid"
SCRIPT="scripts/x200-b44-unicode-font-augment.sh"

fail() { printf 'x200-b44-unicode-font-augment-verbose: %s\n' "$1" >&2; exit 1; }
[ -n "$USB" ] || fail "usage: sh scripts/x200-b44-unicode-font-augment-verbose.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
[ -f "$SCRIPT" ] || fail "missing $SCRIPT"
mkdir -p "$OUT_DIR"
: > "$LOG"

printf 'PHASE1 B44 Unicode/font augmentation — verbose runner\n'
printf 'target : %s\n' "$USB"
printf 'script : %s\n' "$SCRIPT"
printf 'log    : %s\n' "$LOG"
printf '\nThis can take a while while fonts are copied and the initramfs is repacked.\n'
printf 'Progress will print every 5 seconds.\n\n'

(
  sh "$SCRIPT" "$USB" YES_WRITE_USB
) > "$LOG" 2>&1 &
PID=$!
printf '%s\n' "$PID" > "$PID_FILE"

while kill -0 "$PID" 2>/dev/null; do
  fonts=0
  rootfs="$OUT_DIR/work/rootfs"
  if [ -d "$rootfs/usr/share/fonts" ]; then
    fonts="$(find "$rootfs/usr/share/fonts" -type f 2>/dev/null | wc -l | awk '{print $1}')"
  fi
  initrd_size="pending"
  if [ -f "$OUT_DIR/work/phase1-b42-native-stable-safe-color-utf8.img" ]; then
    initrd_size="$(ls -lh "$OUT_DIR/work/phase1-b42-native-stable-safe-color-utf8.img" 2>/dev/null | awk '{print $5}')"
  fi
  printf '[B44 progress] pid=%s copied-font-files=%s repacked-initrd=%s\n' "$PID" "$fonts" "$initrd_size"
  tail -n 4 "$LOG" 2>/dev/null || true
  sleep 5
done

set +e
wait "$PID"
RC=$?
set -e
rm -f "$PID_FILE" 2>/dev/null || true

printf '\n--- B44 final log tail ---\n'
tail -n 80 "$LOG" 2>/dev/null || true
printf -- '--- end B44 final log tail ---\n'

if [ "$RC" -ne 0 ]; then
  fail "augmentation failed with exit code $RC. Full log: $LOG"
fi

printf '\nDONE: verbose B44 augmentation completed successfully.\n'
printf 'Full log: %s\n' "$LOG"
