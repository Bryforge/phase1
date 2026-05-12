#!/usr/bin/env sh
# Phase1 B45 minimal Unicode/Japanese augmentation.
#
# Purpose:
#   Keep the next test small. Package one best font candidate, one renderer
#   candidate, and short Japanese/rounded test strings.
#
# Usage:
#   sh scripts/x200-b45-minimal-unicode-augment.sh /dev/sdb YES_WRITE_USB

set -eu

USB="${1:-}"
CONFIRM="${2:-}"
INITRD_NAME="phase1-b42-native-stable-safe-color-utf8.img"
OUT_DIR="${BASE1_B45_OUT:-build/base1-b45-minimal-unicode}"
WORK="$OUT_DIR/work"
ROOTFS="$WORK/rootfs"
REPORT="$OUT_DIR/b45-minimal-unicode.env"
ENTRY="Start Phase1 Minimal Japanese Glyph Test"

fail() { printf 'x200-b45-minimal-unicode-augment: %s\n' "$1" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
part1() { case "$1" in /dev/nvme[0-9]n[0-9]|/dev/mmcblk[0-9]) printf '%sp1\n' "$1" ;; /dev/sd[a-z]|/dev/vd[a-z]|/dev/hd[a-z]) printf '%s1\n' "$1" ;; *) fail "use a whole disk like /dev/sdb" ;; esac; }
copy_one() { src="$1"; dstroot="$2"; [ -e "$src" ] || return 0; dst="$dstroot$src"; mkdir -p "$(dirname "$dst")"; cp -aL "$src" "$dst" 2>/dev/null || cp -L "$src" "$dst" 2>/dev/null || true; }
copy_libs() { bin="$1"; dstroot="$2"; command -v ldd >/dev/null 2>&1 || return 0; ldd "$bin" 2>/dev/null | while IFS= read -r line || [ -n "${line:-}" ]; do lib=""; case "$line" in *'=> /'*) lib="$(printf '%s\n' "$line" | awk '{print $3}')" ;; /*) lib="$(printf '%s\n' "$line" | awk '{print $1}')" ;; esac; [ -n "$lib" ] && [ -e "$lib" ] && copy_one "$lib" "$dstroot"; done; }
find_font() {
  for pattern in \
    '*unifont*.ttf' '*unifont*.otf' '*unifont*.pcf*' '*unifont*.psf*' \
    '*NotoSansMonoCJK*JP*.otf' '*NotoSansCJK*JP*.otf' '*Noto*Sans*CJK*JP*' \
    '*DejaVuSansMono*.ttf'
  do
    find /usr/share/fonts /usr/share/consolefonts -type f -iname "$pattern" 2>/dev/null | head -n 1
  done | awk 'NF { print; exit }'
}
find_renderer() {
  for tool in /usr/bin/fbterm /usr/bin/jfbterm /usr/bin/kmscon /usr/bin/bterm; do
    [ -x "$tool" ] && { printf '%s\n' "$tool"; return; }
  done
  printf 'linux-vt\n'
}

[ -n "$USB" ] || fail "usage: sh scripts/x200-b45-minimal-unicode-augment.sh /dev/sdb YES_WRITE_USB"
[ "$CONFIRM" = "YES_WRITE_USB" ] || fail "missing YES_WRITE_USB confirmation"
for c in sudo mount umount mkdir cp grep awk find cpio gzip tee mktemp findmnt head; do need "$c"; done
[ -b "$USB" ] || fail "not a block device: $USB"
ROOT_SRC="$(findmnt -no SOURCE / 2>/dev/null || true)"
case "$ROOT_SRC" in "$USB"|"$USB"[0-9]*|"$USB"p[0-9]*) fail "refusing root filesystem device: $ROOT_SRC" ;; esac

FONT="$(find_font || true)"
RENDERER="$(find_renderer)"
[ -n "$FONT" ] || FONT=none

PART="$(part1 "$USB")"
[ -b "$PART" ] || fail "partition not found: $PART"
MNT="$(mktemp -d)"
cleanup() { sudo umount "$MNT" 2>/dev/null || true; rmdir "$MNT" 2>/dev/null || true; }
trap cleanup EXIT INT TERM

mkdir -p "$OUT_DIR"
sudo mount "$PART" "$MNT"
INITRD="$MNT/boot/phase1/$INITRD_NAME"
[ -f "$INITRD" ] || fail "missing initramfs on USB: /boot/phase1/$INITRD_NAME"
[ -f "$MNT/boot/grub/grub.cfg" ] || fail "missing grub.cfg on USB"

rm -rf "$WORK"
mkdir -p "$ROOTFS"
cp "$INITRD" "$WORK/original.img.gz"
( cd "$ROOTFS" && gzip -dc "../original.img.gz" | cpio -idmu 2>/dev/null ) || fail "could not extract initramfs"

mkdir -p "$ROOTFS/phase1/fonts" "$ROOTFS/phase1/i18n/ja" "$ROOTFS/phase1/evidence"
if [ "$FONT" != none ]; then
  copy_one "$FONT" "$ROOTFS"
  cp -aL "$FONT" "$ROOTFS/phase1/fonts/minimal-font" 2>/dev/null || true
fi
if [ "$RENDERER" != linux-vt ]; then
  copy_one "$RENDERER" "$ROOTFS"
  copy_libs "$RENDERER" "$ROOTFS" || true
fi
for helper in /usr/bin/setfont /bin/setfont /usr/bin/unicode_start /bin/unicode_start; do
  [ -x "$helper" ] && { copy_one "$helper" "$ROOTFS"; copy_libs "$helper" "$ROOTFS" || true; }
done

cat > "$ROOTFS/phase1/i18n/ja/minimal-test.txt" <<'EOF'
こんにちは、ハッカー
╭─ Phase1 ─╮
╰──────────╯
EOF
cat > "$ROOTFS/phase1/evidence/b45-minimal-unicode.env" <<EOF
BASE1_B45_MINIMAL_FONT=$FONT
BASE1_B45_MINIMAL_RENDERER=$RENDERER
BASE1_B45_TEST_STRING=hello-hacker-ja
BASE1_B45_JAPANESE_GLYPH_RENDERING=not_claimed
BASE1_B45_RESULT=packaged
EOF

if ! grep -q 'B45 minimal Unicode test' "$ROOTFS/init"; then
  tmp="$ROOTFS/init.b45"
  awk '
    /^exec <\/dev\/console/ && !done {
      print "# B45 minimal Unicode test";
      print "export PHASE1_B45_MINIMAL_UNICODE=1";
      print "export PHASE1_CJK_GLYPH_SUPPORT=minimal-attempt";
      print "[ -x /usr/bin/unicode_start ] && /usr/bin/unicode_start >/dev/null 2>&1 || true";
      print "[ -x /bin/unicode_start ] && /bin/unicode_start >/dev/null 2>&1 || true";
      done=1;
    }
    { print }
  ' "$ROOTFS/init" > "$tmp"
  mv "$tmp" "$ROOTFS/init"
  chmod 0755 "$ROOTFS/init"
fi

( cd "$ROOTFS" && find . | cpio -H newc -o 2>/dev/null | gzip -9 > "../$INITRD_NAME" ) || fail "could not repack initramfs"
sudo cp "$WORK/$INITRD_NAME" "$INITRD"

CFG="$MNT/boot/grub/grub.cfg"
if ! grep -q "$ENTRY" "$CFG"; then
  TMP_LOCAL="$WORK/grub.cfg.b45"
  awk -v initrd="$INITRD_NAME" -v entry="$ENTRY" '
    /menuentry "Start Phase1 ASCII Safe Fallback"/ && !done {
      print "menuentry \"" entry "\" {";
      print "    clear";
      print "    echo \"phase1 6.0.0 ready\"";
      print "    echo \"B45 minimal Japanese glyph test\"";
      print "    linux /boot/phase1/vmlinuz console=tty0 rdinit=/init init=/init vga=791 video=1024x768-16 quiet loglevel=0 panic=0 phase1.stable=1 phase1.safe=1 phase1.ascii=0 phase1.autoboot=0 phase1.utf8=1 phase1.b45_minimal_unicode=1";
      print "    initrd /boot/phase1/" initrd;
      print "    boot";
      print "}";
      print "";
      done=1;
    }
    { print }
  ' "$CFG" > "$TMP_LOCAL"
  sudo cp "$TMP_LOCAL" "$CFG"
fi

cat > "$WORK/b45-prep-append.env" <<EOF
BASE1_B45_MINIMAL_FONT=$FONT
BASE1_B45_MINIMAL_RENDERER=$RENDERER
BASE1_B45_TEST_ENTRY=$ENTRY
BASE1_B45_JAPANESE_GLYPH_RENDERING=not_claimed
EOF
sudo sh -c "cat '$WORK/b45-prep-append.env' >> '$MNT/phase1/evidence/b42-prep.env'"

grep -q "$ENTRY" "$CFG" || fail "minimal Unicode entry was not written"
sync
sudo umount "$MNT"
rmdir "$MNT"
trap - EXIT INT TERM

cat > "$REPORT" <<EOF
BASE1_B45_TARGET=$USB
BASE1_B45_PARTITION=$PART
BASE1_B45_INITRD=$INITRD_NAME
BASE1_B45_MINIMAL_FONT=$FONT
BASE1_B45_MINIMAL_RENDERER=$RENDERER
BASE1_B45_TEST_ENTRY=$ENTRY
BASE1_B45_JAPANESE_GLYPH_RENDERING=not_claimed
BASE1_B45_RESULT=augmented
EOF

printf 'DONE: B45 minimal Unicode augmentation complete.\n'
printf 'Font: %s\n' "$FONT"
printf 'Renderer: %s\n' "$RENDERER"
printf 'Entry: %s\n' "$ENTRY"
printf 'Report: %s\n' "$REPORT"
