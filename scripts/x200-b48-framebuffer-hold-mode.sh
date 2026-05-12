#!/usr/bin/env sh
# Phase1/Base1 B48 framebuffer hold-mode patch.
#
# Purpose:
#   The B47 framebuffer card can be drawn and then immediately overwritten by
#   the native terminal UI. This patch makes the framebuffer entry a proof path:
#   when phase1.framebuffer=1 is present, draw the card, hold it on screen, then
#   drop to shell instead of launching the normal Phase1 terminal card.
#
# Usage after the normal B47 USB prep/augment:
#   sh scripts/x200-b48-framebuffer-hold-mode.sh /dev/sdb YES_WRITE_USB

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
INITRD_NAME="phase1-b42-native-stable-safe-color-utf8.img"
OUT_DIR="${BASE1_B48_OUT:-build/base1-b48-framebuffer-hold-mode}"
WORK="$OUT_DIR/work"
ROOTFS="$WORK/rootfs"
REPORT="$OUT_DIR/b48-framebuffer-hold-mode.env"

fail() { printf 'x200-b48-framebuffer-hold-mode: %s\n' "$1" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
part1() { case "$1" in /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;; /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;; *) fail "use a whole disk like /dev/sdb" ;; esac; }

[ -n "$USB" ] || fail "usage: sh scripts/x200-b48-framebuffer-hold-mode.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
for c in sudo mount umount mkdir cp grep awk cpio gzip mktemp findmnt; do need "$c"; done
[ -b "$USB" ] || fail "not a block device: $USB"
ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing root filesystem device: $ROOT_SRC" ;; esac

PART="$(part1 "$USB")"
[ -b "$PART" ] || fail "partition not found: $PART"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
sudo mount "$PART" "$MNT"
INITRD="$MNT/boot/phase1/$INITRD_NAME"
[ -f "$INITRD" ] || fail "missing initramfs on USB: /boot/phase1/$INITRD_NAME"

rm -rf "$WORK"
mkdir -p "$ROOTFS"
cp "$INITRD" "$WORK/original.img.gz"
( cd "$ROOTFS" && gzip -dc "../original.img.gz" | cpio -idmu 2>/dev/null ) || fail "could not extract initramfs"

[ -x "$ROOTFS/phase1/phase1_fb_blit" ] || fail "missing /phase1/phase1_fb_blit in initramfs; run B47 augment first"
[ -f "$ROOTFS/phase1/phase1-b47-x200-card.ppm" ] || fail "missing framebuffer card in initramfs; run B47 augment first"

# Remove stale hold block if present.
if grep -q 'B48 framebuffer hold mode BEGIN' "$ROOTFS/init"; then
  awk '
    /# B48 framebuffer hold mode BEGIN/ { skip=1; next }
    /# B48 framebuffer hold mode END/ { skip=0; next }
    !skip { print }
  ' "$ROOTFS/init" > "$ROOTFS/init.clean"
  mv "$ROOTFS/init.clean" "$ROOTFS/init"
fi

TMP="$ROOTFS/init.b48"
awk '
  /^exec <\/dev\/console/ && !done {
    print;
    print "# B48 framebuffer hold mode BEGIN";
    print "CMDLINE=\"$(cat /proc/cmdline 2>/dev/null || true)\"";
    print "if echo \" $CMDLINE \" | grep -q \" phase1.framebuffer=1 \"; then";
    print "  clear 2>/dev/null || true";
    print "  echo \"B48_FRAMEBUFFER_HOLD_ACTIVE\"";
    print "  echo \"Preparing framebuffer card; terminal UI will not overwrite this proof path.\"";
    print "  cat /proc/fb 2>/dev/null || true";
    print "  sleep 1";
    print "  if /phase1/phase1_fb_blit /phase1/phase1-b47-x200-card.ppm /dev/fb0; then";
    print "    sleep 25";
    print "  else";
    print "    echo \"B48 framebuffer draw failed; diagnostics follow.\"";
    print "    dmesg | grep -i fb 2>/dev/null || true";
    print "    sleep 10";
    print "  fi";
    print "  clear 2>/dev/null || true";
    print "  echo \"B48 framebuffer proof path complete. Shell fallback active.\"";
    print "  exec /bin/sh";
    print "fi";
    print "# B48 framebuffer hold mode END";
    done=1;
    next;
  }
  { print }
' "$ROOTFS/init" > "$TMP"
mv "$TMP" "$ROOTFS/init"
chmod 0755 "$ROOTFS/init"

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../$INITRD_NAME" ) || fail "could not repack initramfs"
sudo cp "$WORK/$INITRD_NAME" "$INITRD"

VERIFY_ROOT="$WORK/verify-rootfs"
rm -rf "$VERIFY_ROOT"
mkdir -p "$VERIFY_ROOT"
( cd "$VERIFY_ROOT" && gzip -dc "$INITRD" | cpio -idmu 2>/dev/null ) || fail "could not verify extract augmented initramfs"
grep -q 'B48_FRAMEBUFFER_HOLD_ACTIVE' "$VERIFY_ROOT/init" || fail "B48 hold marker missing after repack"
grep -q 'exec /bin/sh' "$VERIFY_ROOT/init" || fail "B48 shell fallback missing after repack"

if [ -f "$MNT/phase1/evidence/b42-prep.env" ]; then
  printf '%s\n' \
    'BASE1_B48_FRAMEBUFFER_HOLD_MODE=1' \
    'BASE1_B48_FRAMEBUFFER_OVERWRITE_PREVENTION=1' \
    | sudo tee -a "$MNT/phase1/evidence/b42-prep.env" >/dev/null
fi
sync
cleanup
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B48_TARGET=$USB
BASE1_B48_PARTITION=$PART
BASE1_B48_FRAMEBUFFER_HOLD_MODE=1
BASE1_B48_FRAMEBUFFER_OVERWRITE_PREVENTION=1
BASE1_B48_RESULT=augmented
EOF

printf 'DONE: B48 framebuffer hold mode applied.\n'
printf 'On framebuffer boot, expect B48_FRAMEBUFFER_HOLD_ACTIVE, then the card should remain visible for 25 seconds.\n'
printf 'Report: %s\n' "$REPORT"
