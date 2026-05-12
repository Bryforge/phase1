#!/usr/bin/env sh
# Force the B47 framebuffer entry to be the first/default GRUB entry on an
# already prepared Phase1 USB.
#
# Usage:
#   sh scripts/x200-b47-force-framebuffer-default.sh /dev/sdb YES_WRITE_USB
#
# Purpose:
#   Stop the test loop from accidentally booting the normal terminal entry.
#   This script mounts the USB, verifies the B47 framebuffer entry exists,
#   rewrites grub.cfg so the framebuffer entry is first/default, and verifies
#   the result by readback.

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
ENTRY="Start Phase1 Framebuffer Boot Card"
PART=""
MNT=""

fail() { printf 'x200-b47-force-framebuffer-default: %s\n' "$1" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
part1() { case "$1" in /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;; /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;; *) fail "use a whole disk like /dev/sdb" ;; esac; }
cleanup() { [ -n "$MNT" ] && sudo umount "$MNT" 2>/dev/null || true; [ -n "$MNT" ] && rmdir "$MNT" 2>/dev/null || true; }

[ -n "$USB" ] || fail "usage: sh scripts/x200-b47-force-framebuffer-default.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
for c in sudo mount umount mktemp grep awk cp findmnt; do need "$c"; done
[ -b "$USB" ] || fail "not a block device: $USB"
ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing root filesystem device: $ROOT_SRC" ;; esac

PART="$(part1 "$USB")"
[ -b "$PART" ] || fail "partition not found: $PART"
MNT="$(mktemp -d)"
trap cleanup EXIT INT TERM
sudo mount "$PART" "$MNT"
CFG="$MNT/boot/grub/grub.cfg"
[ -f "$CFG" ] || fail "missing grub.cfg"

grep -q "$ENTRY" "$CFG" || fail "B47 framebuffer entry not present; run x200-b47-framebuffer-boot-augment.sh first"
grep -q 'phase1.framebuffer=1' "$CFG" || fail "B47 framebuffer kernel flag missing"

TMP_LOCAL="$(mktemp)"
# Remove any existing B47 entry, capture it, then prepend it as the first entry.
awk -v entry="$ENTRY" '
  BEGIN { in_b47=0; depth=0; captured=""; rest="" }
  /^menuentry / {
    if ($0 ~ entry) { in_b47=1; depth=0; captured = captured $0 "\n"; next }
  }
  in_b47 {
    captured = captured $0 "\n";
    if ($0 ~ /\{/) depth++;
    if ($0 ~ /\}/) {
      depth--;
      if (depth <= 0) in_b47=0;
    }
    next;
  }
  { rest = rest $0 "\n" }
  END {
    print "set default=0";
    print "set timeout=8";
    print "set timeout_style=menu";
    print "";
    printf "%s", captured;
    print "";
    printf "%s", rest;
  }
' "$CFG" > "$TMP_LOCAL"

sudo cp "$TMP_LOCAL" "$CFG"
rm -f "$TMP_LOCAL"

# Append evidence marker.
if [ -f "$MNT/phase1/evidence/b42-prep.env" ]; then
  printf '%s\n' \
    'BASE1_B47_FRAMEBUFFER_FORCED_DEFAULT=1' \
    'BASE1_B47_GRUB_DEFAULT_ENTRY=Start Phase1 Framebuffer Boot Card' \
    | sudo tee -a "$MNT/phase1/evidence/b42-prep.env" >/dev/null
fi
sync

# Readback verify before unmounting.
first_menu="$(grep '^menuentry ' "$CFG" | head -n 1 || true)"
printf 'First menu entry: %s\n' "$first_menu"
printf '%s\n' "$first_menu" | grep -q "$ENTRY" || fail "B47 framebuffer entry is not first after rewrite"
grep -q 'set default=0' "$CFG" || fail "GRUB default not set to 0"
grep -q 'set timeout_style=menu' "$CFG" || fail "GRUB menu timeout style not set"

cleanup
trap - EXIT INT TERM

printf 'DONE: B47 framebuffer entry is now first/default on USB.\n'
printf 'On next boot, choose or wait for: %s\n' "$ENTRY"
